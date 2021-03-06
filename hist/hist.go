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

func (H *Hist) Add(v float64) int64 {
	i := int64(v / H.delta)
	H.imap[i] = H.imap[i] + 1
	return i
}

func (H *Hist) Adds(vs []float64) int64 {
	var i int64
	for _, v := range vs {
		i = int64(v / H.delta)
		H.imap[i] = H.imap[i] + 1
	}
	return i
}

func (H Hist) Imap() map[int64]uint64 {
	return H.imap
}

func (H Hist) Kmap() map[float64]uint64 {
	Kmap := make(map[float64]uint64)
	d := H.delta
	for k, v := range H.imap {
		Kmap[float64(k)*d] = v
	}

	return Kmap
}

func Keys(map[float64]uint64) []float64 {

}

func (H Hist) Krecords(p1 func(float64) string, p2 func(uint64) string) [][]string {
	rec := [][]string{}
	for k, v := range H.Kmap() {
		rec = append(rec, []string{
			p1(k),
			p2(v),
		})

	}
	return rec

}
