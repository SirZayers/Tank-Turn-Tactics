# Connection to Server

- TCP

  - fixed port
  - as soon as connection is open, you can send all messages:
    - json message:
      - kid: player id
      - aid: action id
      - action: what action
    - digital signature
  - server can send responses:
    - response
  - per connection: requests and responses are in order

- UDP

  - just send any message (with same format), any time
  - server can send responses back:
    - but: may have different order

- TCP + ssl
  - you know that you are connected to the right server, and no one can read your messagges .\_.

-> for the beginning TCP
