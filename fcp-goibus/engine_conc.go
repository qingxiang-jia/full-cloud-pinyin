package main

import (
	"fmt"
	"sync"

	"github.com/godbus/dbus/v5"
	"github.com/haunt98/goibus/ibus"
)

type State struct {
	preedit     []rune
	candidates  []string
	matchedLen  []int
	ltVisible   bool
	englishMode bool
	depth       int
}

type DBusState struct {
	conn    *dbus.Conn
	objPath *dbus.ObjectPath
}

type IBusState struct {
	prop     *ibus.Property
	propList *ibus.PropList
}

type FcpConcEngine struct {
	ibus.Engine
	mu        sync.Mutex
	cp        *CloudPinyin
	dbusState DBusState
	ibusState IBusState
	now       *State
	level     [8]int
	lt        *ibus.LookupTable
}

func NewFcpConcEngine(conn *dbus.Conn, path *dbus.ObjectPath, prop *ibus.Property) *FcpConcEngine {
	return &FcpConcEngine{
		Engine: ibus.BaseEngine(conn, *path),
		cp:     NewCloudPinyin(),
		dbusState: DBusState{
			conn:    conn,
			objPath: path,
		},
		ibusState: IBusState{
			prop:     prop,
			propList: ibus.NewPropList(prop),
		},
		now: &State{
			preedit:     []rune{},
			candidates:  []string{},
			matchedLen:  []int{},
			ltVisible:   false,
			englishMode: false,
			depth:       0,
		},
		level: [8]int{CandCntA, CandCntB, CandCntC, CandCntD, CandCntE, CandCntF, CandCntG, CandCntH},
		lt:    ibus.NewLookupTable(),
	}
}

func (e *FcpConcEngine) ProcessKeyEvent(keyVal uint32, keyCode uint32, state uint32) (bool, *dbus.Error) {
	key := rune(keyVal)
	fmt.Println(key, string(key))

	// Decides if we need to switch to or out of English mode
	if state == IBusButtonUp && (key == IBusShiftL || key == IBusShiftR) {
		next := State{
			preedit:     []rune{},
			candidates:  []string{},
			matchedLen:  []int{},
			ltVisible:   false,
			englishMode: !e.now.englishMode,
			depth:       0,
		}
		e.applyStateAtomic(&next)
		return true, nil
	}

	// Pinyin typing mode
	if state == IBusButtonDown && !e.now.englishMode {
		// a-z
		if IBusA <= key && key <= IBusZ {
			go func() {
				next := *(e.now)
				next.preedit = append(next.preedit, key)
				cand, matchedLen, err := e.cp.GetCandidates(string(next.preedit), e.level[0])

				if err != nil {
					return
				}

				next.candidates = cand
				next.matchedLen = matchedLen
				next.ltVisible = true
				next.englishMode = false
				next.depth = 0

				e.applyStateAtomic(&next)
			}()
			return true, nil // Yeah, we can't really tell IBus false or non-nil error
		}

		// Non-typing actions
		if e.now.ltVisible {
			// Remove a character from preedit
			if key == IBusBackspace {
				go func() {
					next := *(e.now)
					next.preedit = next.preedit[:len(next.preedit)-1]
					cand, matchedLen, err := e.cp.GetCandidates(string(next.preedit), e.level[0])

					if err != nil {
						return
					}

					next.candidates = cand
					next.matchedLen = matchedLen
					next.ltVisible = true
					next.englishMode = false
					next.depth = 0

					e.applyStateAtomic(&next)
				}()
				return true, nil
			}

			// Terminate typing
			if key == IBusEsc {
				next := State{
					preedit:     []rune{},
					candidates:  []string{},
					matchedLen:  []int{},
					ltVisible:   false,
					englishMode: e.now.englishMode,
					depth:       0,
				}
				e.applyStateAtomic(&next)
				return true, nil
			}

			// Commit preedit as English alphabets
			if key == IBusEnter {
				next := State{
					preedit:     []rune{},
					candidates:  []string{},
					matchedLen:  []int{},
					ltVisible:   false,
					englishMode: e.now.englishMode,
					depth:       0,
				}

				e.mu.Lock()
				text := string(e.now.preedit)
				e.CommitText(ibus.NewText(text))
				e.mu.Unlock()

				e.applyStateAtomic(&next)
				return true, nil
			}

			// Commit the first candidate in the lookup table
			if key == IBusSpace {
				next := State{
					preedit:     []rune{},
					candidates:  []string{},
					matchedLen:  []int{},
					ltVisible:   false,
					englishMode: e.now.englishMode,
					depth:       0,
				}

				e.mu.Lock()
				candidate := e.lt.Candidates[int(e.lt.CursorPos)].Value().(ibus.Text)
				e.CommitText(&candidate)
				e.mu.Unlock()

				e.applyStateAtomic(&next)
				return true, nil
			}

			// Commit the candidate indexed in the lookup table
			if IBus0 <= key && key <= IBus9 {
				idx := int(key) - 48 - 1

				e.mu.Lock() // So others wait for access/modification to e.lt
				next := *(e.now)
				base := int(e.lt.CursorPos / e.lt.PageSize * e.lt.PageSize)
				idx += base
				if 0 <= idx && idx < len(e.lt.Candidates) {
					candidate := e.lt.Candidates[idx].Value().(ibus.Text)
					e.CommitText(&candidate)
				}
				e.mu.Unlock()

				matched := next.matchedLen[idx]
				next.preedit = next.preedit[0:matched]

				if len(next.preedit) == 0 {
					next.candidates = []string{}
					next.matchedLen = []int{}
					next.ltVisible = false
					// englishMode unchanged
					next.depth = 0

					e.applyStateAtomic(&next)
					return true, nil
				}

				// Partial match, still need to get new candidates
				go func() {
					cand, matchedLen, err := e.cp.GetCandidates(string(next.preedit), e.level[0])

					if err != nil {
						return
					}

					next.candidates = cand
					next.matchedLen = matchedLen
					next.ltVisible = true
					next.englishMode = false
					next.depth = 0

					e.applyStateAtomic(&next)
				}()
			}

			// Cursor up lookup table, not using State since from the State's point of view, nothing changes
			if key == IBusUp || key == IBusRight {
				e.mu.Lock()
				if e.moveCursorDown() {
					e.UpdateLookupTable(e.lt, true)
				}
				e.mu.Unlock()
				return true, nil
			}

			// Cursor down lookup table, not using State since from the State's point of view, nothing changes
			if key == IBusDown || key == IBusLeft {
				e.mu.Lock()
				if e.moveCursorUp() {
					e.UpdateLookupTable(e.lt, true)
				}
				e.mu.Unlock()
				return true, nil
			}

			// + to go to next page
			if key == IBusEqual {
				e.mu.Lock()
				e.movePageUp()
				onLastPage := e.onLastPage()
				e.mu.Unlock()

				if onLastPage {
					next := *(e.now)
					if next.depth < len(e.level) {
						next.depth++

						// Load more candidates
						go func() {
							cand, matchedLen, err := e.cp.GetCandidates(string(next.preedit), e.level[next.depth])

							if err != nil {
								return
							}

							// preedit, ltVisible, englishMode are unchanged
							next.candidates = cand
							next.matchedLen = matchedLen

							e.applyStateAtomic(&next)
						}()
					}
				}
				return true, nil
			}

			// - to go to previous page
			if key == IBusMinus {
				e.mu.Lock()
				e.movePageDown()
				e.mu.Unlock()
				return true, nil
			}
		}
	}

	return false, nil
}

func (e *FcpConcEngine) applyStateAtomic(next *State) {
	e.mu.Lock()
	// Has ltVisible changed? If so, update everything
	if next.ltVisible != e.now.ltVisible {
		e.updatePreedit(&next.preedit, next.ltVisible)
		e.updateLt(&next.candidates, next.ltVisible)
		// IBus doesn't care matchedLen, ltVisible, englishMode, depth, so skip
		e.now = next
		e.mu.Unlock()
		return
	}

	// Has depth changed? If so, update candidates, matchedLen
	if next.depth != e.now.depth {
		// IBus
		e.updateLt(&next.candidates, next.ltVisible)
		// IBus doesn't care matchedLen
		e.now = next
		e.mu.Unlock()
		return
	}

	// Has englishMode changed? If so, update everything
	if next.englishMode != e.now.englishMode {
		e.updatePreedit(&next.preedit, next.ltVisible)
		e.updateLt(&next.candidates, next.ltVisible)
		// IBus doesn't care matchedLen, ltVisible, englishMode, depth, so skip
		e.now = next
		e.mu.Unlock()
		return
	}

	// Has preedit changed? If so, update IBus with changes on preedit, candidates, matchedLen
	if string(next.preedit) != string(e.now.preedit) {
		e.updatePreedit(&next.preedit, next.ltVisible)
		e.updateLt(&next.candidates, next.ltVisible)
		// IBus doesn't care matchedLen so skip
		e.now = next
		e.mu.Unlock()
		return
	}
	e.mu.Unlock()
}

func (e *FcpConcEngine) updatePreedit(preedit *[]rune, visible bool) {
	e.UpdatePreeditText(ibus.NewText(string(*preedit)), uint32(1), visible)
}

func (e *FcpConcEngine) updateLt(new *[]string, visible bool) {
	e.clearLt()
	for _, candidate := range *new {
		e.lt.AppendCandidate(candidate)
	}
	e.UpdateLookupTable(e.lt, visible)
}

func (e *FcpConcEngine) clearLt() {
	e.lt.Candidates = e.lt.Candidates[:0]
	e.lt.Labels = e.lt.Labels[:0]
}

// Not sure why the buil-in cursor moving functions don't work so I need to write my own.
func (e *FcpConcEngine) moveCursorUp() bool {
	if int(e.lt.CursorPos) == len(e.lt.Candidates) {
		return false
	}
	e.lt.CursorPos++
	return true
}

// Not sure why the buil-in cursor moving functions don't work so I need to write my own.
func (e *FcpConcEngine) moveCursorDown() bool {
	if e.lt.CursorPos == 0 {
		return false
	}
	e.lt.CursorPos--
	return true
}

// Workaround, because the IBus side doesn't work.
func (e *FcpConcEngine) movePageUp() {
	sz := int(e.lt.PageSize)
	total := len(e.lt.Candidates)
	nextPos := int(e.lt.CursorPos)
	nextPos += sz
	if nextPos >= total {
		nextPos = total - 1
	}
	if nextPos != int(e.lt.CursorPos) {
		e.lt.CursorPos = uint32(nextPos)
		e.UpdateLookupTable(e.lt, true)
	}
}

// Workaround, because the IBus side doesn't work.
func (e *FcpConcEngine) movePageDown() {
	sz := e.lt.PageSize
	pos := e.lt.CursorPos
	if pos < sz {
		return
	}
	pos -= sz
	e.lt.CursorPos = pos
	e.UpdateLookupTable(e.lt, true)
}

func (e *FcpConcEngine) onLastPage() bool {
	sz := int(e.lt.PageSize)
	total := len(e.lt.Candidates)
	maxIdx := (total/sz+1)*sz - 1
	curIdx := int(e.lt.CursorPos)
	if maxIdx-curIdx < sz {
		return true
	} else {
		return false
	}
}
