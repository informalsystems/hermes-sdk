
# Inter-Blockchain Communication (IBC) and Relaying

For readers who are new to the project, [IBC](https://www.ibcprotocol.dev/) is a protocol
that enables secure communication between two blockchains in a permissionless way.
IBC mirrors the concepts of networking protocols, with each message sent from one
chain to another being represented as an _IBC packet_.
At a high level, we can think of blockchains as _pure state machines_ that have no access
to external I/O. To facilitate communication between two blockchains, a _relayer_ is
used to deliver the IBC packets from a _source_ chain to a _destination_ chain.

We can think of IBC relaying similar to mail delivery in the real life. Consider the
case which Alice wants to send a letter to Bob. She would first put her letter in
an envelop, and write down the sender and recipient address. The letter is placed
in a mailbox at Alice's home, which is picked up by a mailman who delivers it
to Bob's home. In the case of IBC, Alice and Bob would be two chains A and B,
and the mailman would be an IBC relayer. The envelop would be an IBC packet,
and the mailboxes would be _provable storage locations_ on the respective chains.

Although the concept of IBC relaying is relatively simple, similar to real world
mail delivery, complexity arises when there are many packets need to be delivered.
There are many cross-cutting concerns in IBC relaying, including latency
(time for a packet to be delivered), throughput (number of packets delivered per timeframe),
reliability (failure recovery), efficiency (avoid delivering the same packet multiple times),
and cost (batch delivery to reduce transaction cost). Instead of choosing a specific
strategy, Hermes SDK allows different relaying strategies to be implemented to
balance different trade offs.
