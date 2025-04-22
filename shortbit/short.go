/**
this is what it looks like in Go
**/

package short

import (
	"fmt"
	"math"
	"math/big"
)

// python'a __init__  method is covered  by the struct and Load method

// Decoder converts bytes to a list of bits.
type shortBit struct {
	idx  uint
	bits string
}

// Load raw bytes and convert to bits
func (sb *shortBit) load(bites []byte) {sb
	i := new(big.Int)
	i.SetBytes(bites)
	sb.bits = fmt.Sprintf("%b", i)
	sb.idx = 0
}

// Strong typing means we need  8,16,32, and 64 bit int methods.

// chunk slices bitcount of bits and returns it as a uint64
func (sb *shortBit) chunk(bitcount uint) *big.Int {
	j := new(big.Int)
	d := sb.idx + bitcount
	if d <= uint(len(sb.bits)){
		j.SetString(sb.bits[sb.idx:d], 2)
		sb.idx = d
	}else{
		j.SetUint64(0)
	}
	return j
}

// asInt8 trims uint64 to 8 bits
func (sb *shortBit) asInt8(bitcount uint) uint8 {
	j := sb.asInt64(bitcount)
	return uint8(j)

}

// asInt16 trims uint64 to 16 bits
func (sb *shortBit) asInt16(bitcount uint) uint16 {
	j := sb.asInt64(bitcount)
	return uint16(j)

}

// asInt32 trims uint64 to 32 bits
func (sb *shortBit) asInt32(bitcount uint) uint32 {
	j := sb.asInt64(bitcount)
	return uint32(j)

}

// asInt64 is a wrapper for chunk
func (sb *shortBit) asInt64(bitcount uint) uint64 {
	j := sb.chunk(bitcount)
	return j.Uint64()

}

// python's as_flag

// asFlag slices 1 bit and returns true for 1 , false for 0
func (sb *shortBit) asFlag() bool {
	var bitcount uint
	bitcount = 1
	j := sb.asInt64(bitcount)
	return j == 1
}

// as_hex()

// asHex slices bitcount of bits and returns as hex string
func (sb *shortBit) asHex(bitcount uint) string {
	j := sb.asInt64(bitcount)
	ashex := fmt.Sprintf("%#x", j)
	return ashex
}

// as_bytes
// asBytes slices bitcount of bits and returns as []bytes
func (sb *shortBit) asBytes(bitcount uint) []byte {
	j := sb.chunk(bitcount)
	return j.Bytes()
}


// similar to as_charset. It's ascii because nobody has asked for other charsets yet.

// asAscii returns the ascii chars of Bytes
func (sb *shortBit) asAscii(bitcount uint) string {
	return string(sb.asBytes(bitcount))
}

// forward.
// goForward advances g.idx by bitcount
func (sb *shortBit) goForward(bitcount uint) {
	sb.idx += bitcount
}
