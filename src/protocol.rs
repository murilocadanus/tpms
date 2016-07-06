// use nom::{Producer,Consumer,ConsumerState,Input,Move,MemProducer,IResult,HexDisplay};

// #[derive(PartialEq,Eq,Debug)]
// enum State {
// 	LogOn,
// 	LogOnAck,
// 	Frame,
// 	FrameAck,
// 	LogOff,
// 	LogOffAck
// }

// struct ProtocolConsumer {
// 	state:		State,
// 	c_state:	ConsumerState<usize,(),Move>,
// 	counter:	usize,
// }

// impl<'a> Consumer<&'a[u8], usize, (), Move> for ProtocolConsumer {
// 	fn state(&self) -> &ConsumerState<usize,(),Move> {
// 		&self.c_state
// 	}

// 	fn handle(&mut self, input: Input<&'a [u8]>) -> &ConsumerState<usize,(),Move> {
// 		match self.state {
// 			State::Beginning => {
// 				match input {
// 						Input::Empty | Input::Eof(None) => {
// 							self.state   = State::Error;
// 							self.c_state = ConsumerState::Error(());
// 						},
// 						Input::Element(sl) | Input::Eof(Some(sl)) => {
// 							match om_parser(sl) {
// 								IResult::Error(_)      => {
// 									self.state   = State::Error;
// 									self.c_state = ConsumerState::Error(());
// 								},
// 								IResult::Incomplete(n) => {
// 									self.c_state = ConsumerState::Continue(Move::Await(n));
// 								},
// 								IResult::Done(i,_)     => {
// 									self.state = State::Middle;
// 									self.c_state = ConsumerState::Continue(Move::Consume(sl.offset(i)));
// 								}
// 							}
// 						}
// 					}
// 			},
// 			State::Middle    => {
// 				match input {
// 					Input::Empty | Input::Eof(None) => {
// 						self.state   = State::Error;
// 						self.c_state = ConsumerState::Error(());
// 					},
// 					Input::Element(sl) | Input::Eof(Some(sl)) => {
// 						match nomnom_parser(sl) {
// 							IResult::Error(_)      => {
// 								self.state   = State::End;
// 								self.c_state = ConsumerState::Continue(Move::Consume(0));
// 							},
// 							IResult::Incomplete(n) => {
// 									println!("Middle got Incomplete({:?})", n);
// 									self.c_state = ConsumerState::Continue(Move::Await(n));
// 							},
// 							IResult::Done(i,noms_vec)     => {
// 								self.counter = self.counter + noms_vec.len();
// 								self.state = State::Middle;
// 								self.c_state = ConsumerState::Continue(Move::Consume(sl.offset(i)));
// 							}
// 						}
// 					}
// 				}
// 			},
// 			State::End       => {
// 				match input {
// 					Input::Empty | Input::Eof(None) => {
// 						self.state   = State::Error;
// 						self.c_state = ConsumerState::Error(());
// 					},
// 					Input::Element(sl) | Input::Eof(Some(sl)) => {
// 						match end_parser(sl) {
// 							IResult::Error(_)      => {
// 								self.state   = State::Error;
// 								self.c_state = ConsumerState::Error(());
// 							},
// 							IResult::Incomplete(n) => {
// 								self.c_state = ConsumerState::Continue(Move::Await(n));
// 							},
// 							IResult::Done(i,_)     => {
// 								self.state = State::Done;
// 								self.c_state = ConsumerState::Done(Move::Consume(sl.offset(i)), self.counter);
// 							}
// 						}
// 					}
// 				}
// 			},
// 			State::Done | State::Error     => {
// 				// this should not be called
// 				self.state = State::Error;
// 				self.c_state = ConsumerState::Error(())
// 			}
// 		};
// 		&self.c_state
// 	}
// }