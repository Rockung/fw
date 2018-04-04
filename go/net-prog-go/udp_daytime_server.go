package main

import (
	"fmt"
	"net"
	"os"
	"time"
)

// func ResolveUDPAddr(net, addr string) (*UDPAddr, error)
// func DialUDP(net string, laddr, raddr *UDPAddr) (c *UDPConn, err error)
// func ListenUDP(net string, laddr *UDPAddr) (c *UDPConn, err error)
// func (c *UDPConn) ReadFromUDP(b []byte) (n int, addr *UDPAddr, err error)
// func (c *UDPConn) WriteToUDP(b []byte, addr *UDPAddr) (n int, err error)

func main() {
	service := ":1200"
	udpAddr, err := net.ResolveUDPAddr("udp", service)
	checkError(err)

	conn, err := net.ListenUDP("udp", udpAddr)
	checkError(err)

	for {
		handleClient(conn)
	}
}

func handleClient(conn *net.UDPConn) {
	var buf [512]byte
	_, addr, err := conn.ReadFromUDP(buf[0:])
	if err != nil {
		return
	}

	daytime := time.Now().String()
	conn.WriteToUDP([]byte(daytime), addr)
}

func checkError(err error) {
	if err != nil {
		fmt.Fprintf(os.Stderr, "Fatal error ", err.Error())
		os.Exit(1)
	}
}
