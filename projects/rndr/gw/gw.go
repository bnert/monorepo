package main

import (
  "fmt"
  "log"
  "net/http"
  "time"
  "github.com/gorilla/mux"
)

func main() {
  r := mux.NewRouter()

  r.HandleFunc("/{type}", func(w http.ResponseWriter, r *http.Request) {
    vars := mux.Vars(r)
    w.WriteHeader(http.StatusOK)
    fmt.Fprintf(w, "Var: %v\n", vars["type"])
  })

  srv := &http.Server{
    Handler: r,
    Addr: "localhost:8087",
    WriteTimeout: 15 * time.Second,
    ReadTimeout: 15 * time.Second,
  }

  log.Fatal(srv.ListenAndServe())
}
