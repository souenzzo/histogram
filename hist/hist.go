package hist

type Hist struct {
	imap  map[int64]uint64
	delta float64
}

func New(d float64) Hist {
	return Hist{
		imap:  make(map[int64]uint64),
		delta: d,
	}
}

func (H *Hist) Add(v float64) (i int64) {
	i = int64(v / H.delta)
	H.imap[i] = H.imap[i] + 1
	return i
}

func (H *Hist) Adds(vs []float64) int {
	for _, v := range vs {
		H.Add(v)
	}
	return len(vs)
}

func (H Hist) Imap() (imap map[int64]uint64) {
	for i, v := range H.imap {
		imap[i] = v
	}
	return imap
}

func (H Hist) Kmap() (kmap map[float64]uint64) {
	for i, v := range H.imap {
		kmap[float64(i)*H.delta] = v
	}
	return kmap
}
