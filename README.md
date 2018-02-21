# etherparse
[![Build Status](https://travis-ci.org/JulianSchmid/etherparse.svg?branch=master)](https://travis-ci.org/JulianSchmid/etherparse)

A library for parsing ethernet & ethernet using protocols.

Currently supported are:
* Ethernet II
* IEEE 802.1Q VLAN Tagging Header
* IPv4
* IPv6 (missing extension headers, but supporting skipping them)

# References
* Darpa Internet Program Protocol Specification [RFC 791](https://tools.ietf.org/html/rfc791)
* Internet Protocol, Version 6 (IPv6) Specification [RFC 8200](https://tools.ietf.org/html/rfc8200)
* [IANA Protocol Numbers](https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml)
* [Wikipedia IEEE_802.1Q](https://en.wikipedia.org/w/index.php?title=IEEE_802.1Q&oldid=820983900)