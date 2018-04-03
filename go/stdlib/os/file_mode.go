package main

import (
  "os"
  "fmt"
  "log"
)

func main() {
  fi, err := os.Lstat("file_mode.go")
  if err != nil {
    log.Fatal(err)
  }

  switch mode := fi.Mode(); {
  case mode.IsRegular():
    fmt.Println("regular file")
  case mode.IsDir():
    fmt.Println("directory")
  case mode & os.ModeSymlink != 0:
    fmt.Println("symbolic link")
  case mode & os.ModeNamedPipe != 0:
    fmt.Println("named pipe")
  }
}
