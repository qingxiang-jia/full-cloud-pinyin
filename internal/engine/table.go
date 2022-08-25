package engine

import "github.com/haunt98/goibus/ibus"

type CandidateTable struct {
	e       *ibus.Engine
	lt      *ibus.LookupTable
	Visible bool
}

func NewCandidateTbale(e *ibus.Engine, lt *ibus.LookupTable) *CandidateTable {
	return &CandidateTable{
		e:       e,
		lt:      ibus.NewLookupTable(),
		Visible: false,
	}
}

func (t *CandidateTable) Add(candidates []string) {
	for _, val := range candidates {
		t.lt.AppendCandidate(val)
	}
	t.e.UpdateLookupTable(t.lt, true)
}

func (t *CandidateTable) Clear() {
	t.lt.Candidates = t.lt.Candidates[:0]
	t.lt.Labels = t.lt.Labels[:0]
}

func (t *CandidateTable) Hide() {
	t.e.HideLookupTable()
	t.Visible = false
}

func (t *CandidateTable) Show() {
	t.e.ShowLookupTable()
	t.Visible = true
}

// Not sure why the buil-in cursor moving functions don't work so I need to write my own.
// Same for the next three.
func (t *CandidateTable) CursorDown() {
	if t.lt.CursorPos == 0 {
		return
	}
	t.lt.CursorPos--
	t.e.UpdateLookupTable(t.lt, true)
}

func (t *CandidateTable) CursorUp() {
	if int(t.lt.CursorPos) == len(t.lt.Candidates) {
		return
	}
	t.lt.CursorPos++
	t.e.UpdateLookupTable(t.lt, true)
}

// Workaround, because the IBus side doesn't work.
func (t *CandidateTable) PageDown() {
	sz := t.lt.PageSize
	pos := t.lt.CursorPos
	if pos < sz {
		return
	}
	pos -= sz
	t.lt.CursorPos = pos
	t.e.UpdateLookupTable(t.lt, true)
}

// Workaround, because the IBus side doesn't work.
func (t *CandidateTable) PageUp() {
	sz := int(t.lt.PageSize)
	total := len(t.lt.Candidates)
	nextPos := int(t.lt.CursorPos)
	nextPos += sz
	if nextPos >= total {
		nextPos = total - 1
	}
	if nextPos != int(t.lt.CursorPos) {
		t.lt.CursorPos = uint32(nextPos)
		t.e.UpdateLookupTable(t.lt, true)
	}
}

func (t *CandidateTable) AtLastPage() bool {
	sz := int(t.lt.PageSize)
	total := len(t.lt.Candidates)
	maxIdx := (total/sz+1)*sz - 1
	curIdx := int(t.lt.CursorPos)
	if maxIdx-curIdx < sz {
		return true
	} else {
		return false
	}
}

func (t *CandidateTable) Cursor() int {
	return int(t.lt.CursorPos)
}

func (t *CandidateTable) PageSize() int {
	return int(t.lt.PageSize)
}
