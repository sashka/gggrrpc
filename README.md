Actors
======


gRPC benchmark:
ghz --proto=go/hello/hello.proto --insecure --call=helloworld.Greeter/SayHello -d '{"name":"GHZ"}' -n20000 [::1]:50051

