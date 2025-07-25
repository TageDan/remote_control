# Remote Input
This is a project to allow you to use you phone as a remote control for a computer. The idea is to run this on a raspberry pi to make it into a custom smart tv (so that i can have ad-blockers and such).

It's simple a webserver listening for commands through a websocket connection wich it will then execute using a virtual input device. The hard part is making actually enjoyable to use. 

Remember to change the `PASSWORD` constant in `main.rs` before compiling! (But it's probably not safe anyways. Just obscure enough for me, since this isn't anything serious at the moment).

