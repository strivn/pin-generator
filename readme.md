# PIN Generator

A simple PIN generator using Rust. Takes the first found n-digits code from several hashes. For example, 

`cargo run generate -p "bank xyz" -d 6` 

Results in
```
The PIN generated value from SHA256 is: 467347
The PIN generated value from SHA384 is: 322577
The PIN generated value from SHA512 is: 339532
The PIN generated value from SHA3_256 is: 293489
The PIN generated value from SHA3_384 is: 225106
The PIN generated value from SHA3_512 is: 593915
```

The main use case is to create some 'random looking' PIN based on a passphrase, but ensures that it's easy enough to recover. 



