
# Architecture

## Arduino

`Arduino` crate does all the computations. It consumes start approximations, epsilon. It provides points for graphics of function. 

Input data:
 - start approximation (range from a to b is defined by physical buttons)
 - epsilon (setup by host)

Output data:
 - function points
 - computed roots

## Protocol

Protocol is based on request-response architecture. 
Only host can make requests. 
Only one response-request round exists at a time.

### Protocol structure

Communication is started by arduino. Arduino sends `PROTOCOL_SIGNATURE`. Host verifies signature. Then host sends requests and arduino sends responses back.

Package format is defined as follows:

| field-name   | type          | size (bytes) | comment                                     |
| ------------ | ------------- | ------------ | ------------------------------------------- |
| package-type | `PackageType` | 0           |                                             |
| payload      | `Payload`     | variadic     | Payload size is defined as per package type |

`PackageType`

Zero size field. Host always sends request and arduino always sends responds

 - `Request`
 - `Response`

`Request`
| field-name   | type          | size (bytes) | comment                                     |
| ------------ | ------------- | ------------ | ------------------------------------------- |
| request-type | `RequestType` | u8           |                                             |
| payload      | `Payload`     | variadic     | Payload size is defined as per package type |

`RequestType`
 - `FunctionPoints` -- points to plot function graphic from
 - `InitialApproximation` -- Requests initial approximation from arduino. Response data is used to draw vertical line. These lines denote initial approximation or borders of interval
 - `SelectMethod` -- instructs arduino to select computation method
 - `ComputeRoot` -- asks arduino to compute root according to specified settings

`SelectMethod`
| field-name | type     | size (bytes) | comment                       |
| ---------- | -------- | ------------ | ----------------------------- |
| method     | `Method` | u8           | Use the `Method` to find root |

`ComputeRoot`
| field-name | type  | size (bytes) | comment            |
| ---------- | ----- | ------------ | ------------------ |
| epsilon    | `f64` | u64          | required precision |

Upon initialization arduino selects default method to its linking. 
Host *MUST* send `SelectMethod` before 
 
`Method`
 - `Chord`
 - `Secant`
 - `SimpleIterationSingle` -- for single non-linear equation
 - `SimpleIteration` -- for non-linear equation *system*

`Response`
| field-name | type              | size (bytes) | comment                                                                                                  |
| ---------- | ----------------- | ------------ | -------------------------------------------------------------------------------------------------------- |
| payload    | `ResponsePayload` | variadic     | Payload size is defined as per package type. Response payload type is determined by current request type |

`ResponsePayload` members have names in the form of `<RequestType>Response`. Example: Request type is `FunctionPoints`. Response type is `FunctionPointsResponse`

`FunctionPointsResponse`
| field-name | type           | size (bytes)       | comment |
| ---------- | -------------- | ------------------ | ------- |
| points     | `[Point; 256]` | sizeof Point * 256 |         |

`Point`
| field-name | type  | size (bytes) | comment                                                           |
| ---------- | ----- | ------------ | ----------------------------------------------------------------- |
| x          | `f64` | u64          | lets just hope that rust supports float points computation of avr |
| y          | `f64` | u64          |                                                                   |

`InitialApproximationResponse`
| field-name | type  | size (bytes) | comment |
| ---------- | ----- | ------------ | ------- |
| left       | `f64` | u64          |         |
| right      | `f64` | u64          |         |

`SelectMethodResponse`
| field-name      | type     | size (bytes) | comment                     |
| --------------- | -------- | ------------ | --------------------------- |
| previous-method | `Method` | u8           | Method, selected previously |


Depending on selected method of computation different interpretation of structure takes place:

If method implies $x_0$ point instead of range, then $x_0$ is placed into `left` field.
For interval, A is placed into `left` and B is placed into `right`.

`ComputeRootResponse`
| field-name | type    | size (bytes) | comment |
| ---------- | ------- | ------------ | ------- |
| root       | `Point` | sizeof Point |         |

## Terms

Host - usb host device. Has vast computations capabilities. Notebook is host in context of the lab.

