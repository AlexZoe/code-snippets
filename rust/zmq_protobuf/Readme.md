## Generate Protobuf
Use the following command in the same directory as the protobuf file.
Alternatively, change the 'rust\_out' directory and add the path to 
the 'proto' file.

```
protoc --rust_out=. sensor_messages.proto
```
