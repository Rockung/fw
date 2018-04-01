package math

import "testing"

// **go install/build** ignore any file that end with _test.go
// which are used only by **go test**.
// eg. go test pkg/math

func TestAverage(t *testing.T) {
  v := Average([]float64{1,2})
  if v != 1.5 {
    t.Error("Expected 1.5, got ", v)
  }
}

type testpair struct {
  values []float64
  average float64
}

var tests = []testpair{
  { []float64{1,2}, 1.5 },
  { []float64{1,1,1,1,1}, 1},
  { []float64{-1,1}, 0},
}

func TestAverage2(t *testing.T) {
  for _, pair := range tests {
    v := Average(pair.values)
    if v != pair.average {
      t.Error(
        "For", pair.values,
        "expected", pair.average,
        "got", v,
      )
    }
  }
}
