# Contracts

There are 2 sample contracts: Ping and Pong contracts. In case a Ping contract is instantiated, a Pong contract is also instantiated.

Pong instantiation is triggered via a sub message (see contract.rs). The Ping contract also has a reply() for handling responses from Pong's instantiation.

# Integration Tests

Ping's integration_tests.rs properly handles instantiation of Ping contract. In return AppResponse holds:

- an instantiation event for Ping
- an instantiation event for Pong
- a successfull reply event from Pong passed to Ping contract
- a response Event containing Pong's contract address.

In total AppResponse contains 6 events:

```JSON
"AppResponse"{
   "events":[
      "Event"{
         "ty":"instantiate",
         "attributes":[
            "Attribute"{
               "key":"_contract_addr",
               "value":"contract0"
            },
            "Attribute"{
               "key":"code_id",
               "value":"1"
            }
         ]
      },
      "Event"{
         "ty":"wasm",
         "attributes":[
            "Attribute"{
               "key":"_contract_addr",
               "value":"contract0"
            },
            "Attribute"{
               "key":"method",
               "value":"instantiate"
            },
            "Attribute"{
               "key":"owner",
               "value":"ADMIN"
            },
            "Attribute"{
               "key":"count",
               "value":"0"
            }
         ]
      },
      "Event"{
         "ty":"instantiate",
         "attributes":[
            "Attribute"{
               "key":"_contract_addr",
               "value":"contract1"
            },
            "Attribute"{
               "key":"code_id",
               "value":"2"
            }
         ]
      },
      "Event"{
         "ty":"wasm",
         "attributes":[
            "Attribute"{
               "key":"_contract_addr",
               "value":"contract1"
            },
            "Attribute"{
               "key":"method",
               "value":"instantiate"
            },
            "Attribute"{
               "key":"owner",
               "value":"contract0"
            },
            "Attribute"{
               "key":"count",
               "value":"0"
            }
         ]
      },
      "Event"{
         "ty":"reply",
         "attributes":[
            "Attribute"{
               "key":"_contract_addr",
               "value":"contract0"
            },
            "Attribute"{
               "key":"mode",
               "value":"handle_success"
            }
         ]
      },
      "Event"{
         "ty":"wasm",
         "attributes":[
            "Attribute"{
               "key":"_contract_addr",
               "value":"contract0"
            },
            "Attribute"{
               "key":"pong address",
               "value":"contract1"
            }
         ]
      }
   ],
   "data":Some(Binary(0a09636f6e747261637430))
}
```

# Cargo workspace

There is a main Cargo.toml file in root folder. It defines a Cargo Workspace and include all members in contracts/*

This allows like building and testing all contracts from root folder:

```bash
$ cargo build # builds all contracts (pint and pong)
$ cargo test # tests all contracts (pint and pong)
$ cargo build -p ping # builds ping contract
$ cargo test -p ping # tests ping contract
```