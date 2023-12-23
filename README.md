# Networking Ideas
Peer-to-peer messaging through a brokered service bus, where agents and subscribe and publish to eachother through the broker server to send, request, and recieve messages.
In an advanced implementation, agents should be able to communicate accross shared memory if they're on the same machine, perhaps DBus, though another cross platform option might be favorable. 
This implementation will be hidden and the exposed messaging interface won't be concerned if it's individual agents are networking to send messages. 
