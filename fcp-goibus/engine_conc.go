package main

import (
	"fmt"
	"sync"

	"github.com/godbus/dbus/v5"
	"github.com/haunt98/goibus/ibus"
)

type FcpConcEngine struct {
	ibus.Engine
	mu          sync.Mutex
	cloud       *CloudPinyin
	propList    *ibus.PropList
	preedit     []rune
	lt          *ibus.LookupTable
	ltVisible   bool
	matchedLen  []int
	englishMode bool
	level       [8]int
	depth       int
}

func NewFcpConcEngine(conn *dbus.Conn, path *dbus.ObjectPath, prop *ibus.Property) *FcpConcEngine {
	return &FcpConcEngine{
		Engine:      ibus.BaseEngine(conn, *path),
		cloud:       NewCloudPinyin(),
		propList:    ibus.NewPropList(prop),
		preedit:     []rune{},
		lt:          ibus.NewLookupTable(),
		ltVisible:   false,
		matchedLen:  []int{},
		englishMode: false,
		level:       [8]int{CandCntA, CandCntB, CandCntC, CandCntD, CandCntE, CandCntF, CandCntG, CandCntH},
		depth:       0,
	}
}

func (e *FcpConcEngine) ProcessKeyEvent(keyVal uint32, keyCode uint32, state uint32) (bool, *dbus.Error) {
	key := rune(keyVal)
	fmt.Println(key, string(key))

	// Decides whether need to switch to/out of English mode
	if state == IBusButtonUp && (key == IBusShiftL || key == IBusShiftR) {
		e.depth = 0
		e.englishMode = !e.englishMode
	}

	if state == IBusButtonDown && !e.englishMode {
		// a-z
		if IBusA <= key && key <= IBusZ {
			e.depth = 0
			e.setPreeditAtomic(append(e.preedit, key))
			preedit := e.preedit

			go func() {
				candidates, matchedLen, err := e.cloud.GetCandidates(string(preedit), CandCntA)
				if err == nil {
					e.setCandidatesAtomic(candidates, matchedLen)
				} else {
					fmt.Println(err)
				}
			}()

			return true, nil
		}

		if e.ltVisible {
			// Remove a character from preedit
			if key == IBusBackspace {
				e.depth = 0

				if len(e.preedit) == 0 {
					e.mu.Lock()
					e.hideLt()
					e.mu.Unlock()
					return true, nil
				} else {
					e.setPreeditAtomic(e.preedit[:len(e.preedit)-1])
				}
				preedit := e.preedit

				go func() {
					candidates, matchedLen, err := e.cloud.GetCandidates(string(preedit), CandCntA)
					if err == nil {
						e.setCandidatesAtomic(candidates, matchedLen)
					} else {
						fmt.Println(err)
					}
				}()

				return true, nil
			}

			// Terminate typing
			if key == IBusEsc {
				e.depth = 0
				e.setPreeditAtomic(e.preedit[:0])
				e.setCandidatesAtomic([]string{}, []int{})

				e.mu.Lock()
				e.hideLt()
				e.mu.Unlock()

				return true, nil
			}

			// Commit preedit as alphabets
			if key == IBusEnter {
				e.depth = 0

				toCommit := e.preedit
				e.commitTextAtomic(toCommit)

				e.setCandidatesAtomic([]string{}, []int{})
				e.setPreeditAtomic([]rune{})

				e.mu.Lock()
				e.hideLt()
				e.mu.Unlock()

				return true, nil
			}

			// Commit a candidate
			if key == IBusSpace {
				e.depth = 0

				idx := int(e.lt.CursorPos)
				e.commitCandidateAtomic(idx)
				matchedLen := e.matchedLen[idx]
				preedit := e.preedit

				if len(e.preedit) > matchedLen {
					e.setPreeditAtomic(preedit[matchedLen:])
					preedit = e.preedit

					go func() {
						candidates, matchedLen, err := e.cloud.GetCandidates(string(preedit), CandCntA)
						if err == nil {
							e.setCandidatesAtomic(candidates, matchedLen)
						} else {
							fmt.Println(err)
						}
					}()
				} else {
					// Full match
					e.setCandidatesAtomic([]string{}, []int{})
					e.setPreeditAtomic([]rune{})
				}

				return true, nil
			}

			// Commit candidate by keying in candidate index
			if IBus0 <= key && key <= IBus9 {
				e.mu.Lock()
				idx := int(key) - 48 - 1
				base := int(e.lt.CursorPos / e.lt.PageSize * e.lt.PageSize)
				idx += base
				candidatesSize := len(e.lt.Candidates)
				e.mu.Unlock()

				if 0 <= idx && idx < candidatesSize {
					e.depth = 0
					e.commitCandidateAtomic(idx)

					matchedLen := e.matchedLen[idx]
					preedit := e.preedit

					if len(e.preedit) > matchedLen {
						e.setPreeditAtomic(preedit[matchedLen:])
						preedit = e.preedit

						go func() {
							candidates, matchedLen, err := e.cloud.GetCandidates(string(preedit), CandCntA)
							if err == nil {
								e.setCandidatesAtomic(candidates, matchedLen)
							} else {
								fmt.Println(err)
							}
						}()
					} else {
						// Full match
						e.setCandidatesAtomic([]string{}, []int{})
						e.setPreeditAtomic([]rune{})
					}
				}

				return true, nil
			}

			// Cursor up lookup table
			if key == IBusUp || key == IBusLeft {
				e.mu.Lock()

				if e.moveCursorDown() {
					e.UpdateLookupTable(e.lt, true)
					e.ltVisible = true
				}

				e.mu.Unlock()

				return true, nil
			}

			// Cursor down lookup table
			if key == IBusDown || key == IBusRight {
				e.mu.Lock()

				if e.moveCursorUp() {
					e.UpdateLookupTable(e.lt, true)
					e.ltVisible = true
				}

				e.mu.Unlock()

				return true, nil
			}

			// + to go to next page
			if key == IBusEqual {
				e.mu.Lock()

				e.movePageUp()
				loadMore := e.atLastPage()

				e.mu.Unlock()

				// We may want to load more candidates
				if loadMore && e.depth < len(e.level) {
					e.depth++
					depth := e.level[e.depth]
					preedit := e.preedit

					go func() {
						candidates, matchedLen, err := e.cloud.GetCandidates(string(preedit), depth)
						if err == nil {
							e.setCandidatesAtomic(candidates, matchedLen)
						} else {
							fmt.Println(err)
						}
					}()
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

func (e *FcpConcEngine) setCandidatesAtomic(candidates []string, matchedLen []int) {
	e.mu.Lock()

	e.clearLt()
	for _, candidate := range candidates {
		e.lt.AppendCandidate(candidate)
	}
	e.matchedLen = matchedLen
	e.UpdateLookupTable(e.lt, true)
	e.ltVisible = true

	e.mu.Unlock()
}

func (e *FcpConcEngine) setPreeditAtomic(preedit []rune) {
	e.mu.Lock()

	e.preedit = preedit
	e.UpdatePreeditText(ibus.NewText(string(e.preedit)), uint32(1), true)
	e.ltVisible = true

	e.mu.Unlock()
}

func (e *FcpConcEngine) commitTextAtomic(text []rune) {
	e.mu.Lock()
	e.CommitText(ibus.NewText(string(text)))
	e.mu.Unlock()
}

func (e *FcpConcEngine) commitCandidateAtomic(i int) {
	e.mu.Lock()

	text := e.lt.Candidates[i].Value().(ibus.Text)
	e.CommitText(&text)

	e.mu.Unlock()
}

// Not sure why the buil-in cursor moving functions don't work so I need to write my own.
// Same for the next three.
func (e *FcpConcEngine) moveCursorUp() bool {
	if int(e.lt.CursorPos) == len(e.lt.Candidates) {
		return false
	}
	e.lt.CursorPos++
	return true
}

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
		e.ltVisible = true
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
	e.ltVisible = true
}

func (e *FcpConcEngine) atLastPage() bool {
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

func (e *FcpConcEngine) hideLt() {
	e.HideLookupTable()
	e.ltVisible = false
}

func (e *FcpConcEngine) showLt() {
	e.ShowLookupTable()
	e.ltVisible = true
}

func (e *FcpConcEngine) clearLt() {
	e.lt.Candidates = e.lt.Candidates[:0]
	e.lt.Labels = e.lt.Labels[:0]
}

// Called when the user clicks a text area
func (e *FcpConcEngine) FocusIn() *dbus.Error {
	e.RegisterProperties(e.propList)
	return nil
}

// Called when any of the UI props are called
func (e *FcpConcEngine) PropertyActivate(prop_name string, prop_state uint32) *dbus.Error {
	return nil
}
