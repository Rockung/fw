package main

import (
	"fmt"
	"io/ioutil"
	"net"
	"os"
)

// type TCPAddr struct {
//   IP   IP
//   Port int
//   Zone string
// }

// func ResolveTCPAddr(net, addr string) (*TCPAddr, error)
// func DialTCP(net string, laddr, raddr *TCPAddr) (c *TCPConn, err error)
// func (c *TCPConn) Write(b []byte) (n int, err error)
// func (c *TCPConn) Read(b []byte) (n int, err error)

func main() {
	if len(os.Args) != 2 {
		fmt.Fprintf(os.Stderr, "Usage: %s host:port ", os.Args[0])
		os.Exit(1)
	}

	service := os.Args[1]
	tcpAddr, err := net.ResolveTCPAddr("tcp4", service)
	checkError(err)

	conn, err := net.DialTCP("tcp", nil, tcpAddr)
	checkError(err)

	_, err = conn.Write([]byte("HEAD / HTTP/1.0\r\n\r\n"))
	checkError(err)

	result, err := ioutil.ReadAll(conn)
	checkError(err)

	fmt.Println(string(result))
	os.Exit(0)
}

func checkError(err error) {
	if err != nil {
		fmt.Fprintf(os.Stderr, "Fatal error: %s", err.Error())
		os.Exit(1)
	}
}
