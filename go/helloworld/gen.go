package helloworld

//go:generate protoc --go_out=. hello.proto
//go:generate protoc --go-grpc_out=. hello.proto

// Prerequisites:
// - protocol buffer compiler, protoc, version 3.
// - go install google.golang.org/protobuf/cmd/protoc-gen-go
// - go install google.golang.org/grpc/cmd/protoc-gen-go-grpc
