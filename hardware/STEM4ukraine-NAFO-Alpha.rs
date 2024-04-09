ha:cschem-sheet-v1 {
	ha:obj_indirect.1 {
		li:objects {
			ha:group.1 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAAAf;
				li:objects {
					ha:group.1 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAAAg; loclib_name=led5;
						li:objects {
						}
						ha:attrib {
							device=led5
							footprint=LED5
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
					ha:group.2 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAAAh; loclib_name=3mmLED_backplane-annotated;
						li:objects {
						}
						ha:attrib {
							footprint=3mmLEDbackplane-annotated.rf
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
				}
				ha:attrib {
					ha:purpose = { value=devmap; prio=0; }
				}
			}
		}
	}
	ha:obj_direct.2 {
		uuid=VBZ6rJ6kY0wH4rUapfwAAAAC;
		li:objects {
			ha:pen.sheet-decor { shape=round; size=125; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.sheet-decor-fill { shape=round; size=125; color=#bbbbbb; font_height=3000; font_family=sans; }
			ha:pen.titlebox-frame { shape=round; size=250; color=#777777; font_height=0; }
			ha:pen.titlebox-fill { shape=round; size=250; color=#bbffbb; font_height=0; }
			ha:pen.titlebox-big { shape=round; size=250; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-small { shape=round; size=250; color=#777777; font_height=1500; font_family=sans; }
			ha:pen.wire { shape=round; size=250; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.bus { shape=round; size=1500; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.hub { shape=round; size=3000; color=#6666ff; font_height=3000; font_family=sans; }
			ha:pen.sym-decor { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; }
			ha:pen.sym-decor-fill { shape=round; size=125; color=#99ff99; font_height=3000; font_family=sans; }
			ha:pen.sym-primary { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.sym-secondary { shape=round; size=125; color=#33bb33; font_height=3000; font_family=sans; }
			ha:pen.term-decor { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.term-primary { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.term-secondary { shape=round; size=250; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.busterm-decor { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.busterm-primary { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.busterm-secondary { shape=round; size=1500; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.junction { shape=round; size=1000; color=#2222bb; font_height=3000; font_family=sans; }
			ha:group.2 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAAAK; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=64000; y=56000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAAAL; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAAAM; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=acy(300)
					name=R1
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=220R
				}
			}
			ha:group.3 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAAAQ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=92000; y=56000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAAAR; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAAAS; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=acy(300)
					name=R2
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=220R
				}
			}
			ha:group.4 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAAAZ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAQ;
				x=64000; y=16000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAAAa; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
							ha:spice/pinnum = { value=2; prio=31050; }
						}
					}
					ha:group.2 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAAAb; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						x=-16000; y=0; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
							ha:spice/pinnum = { value=1; prio=31050; }
						}
					}
					ha:line.3 { x1=-4000; y1=0; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=-12000; y1=0; x2=-10000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=-10000; y1=4000; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-6000; y1=0; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=-10000; y1=4000; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=-6000; y1=4000; x2=-6000; y2=-4000; stroke=sym-decor; }
					ha:text.10 { x1=-8000; y1=13000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.11 { x1=-8000; y1=8000; x2=-6000; y2=11000; stroke=sym-decor; }
					ha:line.12 { x1=-6000; y1=11000; x2=-7000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=-6000; y1=11000; x2=-6517; y2=9545; stroke=sym-decor; }
					ha:line.14 { x1=-10000; y1=7000; x2=-8000; y2=10000; stroke=sym-decor; }
					ha:line.15 { x1=-8000; y1=10000; x2=-8000; y2=8000; stroke=sym-decor; }
					ha:line.16 { x1=-8303; y1=6354; x2=-6303; y2=9354; stroke=sym-decor; }
					ha:line.17 { x1=-6303; y1=9354; x2=-7303; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=-6303; y1=9354; x2=-6820; y2=7899; stroke=sym-decor; }
					ha:line.19 { x1=-10303; y1=5354; x2=-8303; y2=8354; stroke=sym-decor; }
					ha:line.20 { x1=-8303; y1=8354; x2=-8303; y2=6354; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=3mmLED_backplane-annotated
					name=LED1
					role=symbol
					ha:spice/prefix = { value=D; prio=31050; }
				}
			}
			ha:group.5 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAAAc; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAQ;
				x=92000; y=16000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAAAd; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
							ha:spice/pinnum = { value=2; prio=31050; }
						}
					}
					ha:group.2 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAAAe; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						x=-16000; y=0; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
							ha:spice/pinnum = { value=1; prio=31050; }
						}
					}
					ha:line.3 { x1=-4000; y1=0; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=-12000; y1=0; x2=-10000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=-10000; y1=4000; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-6000; y1=0; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=-10000; y1=4000; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=-6000; y1=4000; x2=-6000; y2=-4000; stroke=sym-decor; }
					ha:text.10 { x1=-8000; y1=13000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.11 { x1=-8000; y1=8000; x2=-6000; y2=11000; stroke=sym-decor; }
					ha:line.12 { x1=-6000; y1=11000; x2=-7000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=-6000; y1=11000; x2=-6517; y2=9545; stroke=sym-decor; }
					ha:line.14 { x1=-10000; y1=7000; x2=-8000; y2=10000; stroke=sym-decor; }
					ha:line.15 { x1=-8000; y1=10000; x2=-8000; y2=8000; stroke=sym-decor; }
					ha:line.16 { x1=-8303; y1=6354; x2=-6303; y2=9354; stroke=sym-decor; }
					ha:line.17 { x1=-6303; y1=9354; x2=-7303; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=-6303; y1=9354; x2=-6820; y2=7899; stroke=sym-decor; }
					ha:line.19 { x1=-10303; y1=5354; x2=-8303; y2=8354; stroke=sym-decor; }
					ha:line.20 { x1=-8303; y1=8354; x2=-8303; y2=6354; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=3mmLED_backplane-annotated
					name=LED2
					role=symbol
					ha:spice/prefix = { value=D; prio=31050; }
				}
			}
			ha:group.6 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAACM; src_uuid=VBZ6rJ6kY0wH4rUapfwAAACF;
				x=8000; y=60000; mirx=1; miry=1;
				li:objects {
					ha:text.1 { x1=-2000; y1=-6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAACN; src_uuid=VBZ6rJ6kY0wH4rUapfwAAACG;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAACO; src_uuid=VBZ6rJ6kY0wH4rUapfwAAACH;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.4 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAACP; src_uuid=VBZ6rJ6kY0wH4rUapfwAAACI;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:group.5 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAACQ; src_uuid=VBZ6rJ6kY0wH4rUapfwAAACJ;
						x=0; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:group.6 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAACR; src_uuid=VBZ6rJ6kY0wH4rUapfwAAACK;
						x=0; y=16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=5
							role=terminal
						}
					}
					ha:group.7 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAACS; src_uuid=VBZ6rJ6kY0wH4rUapfwAAACL;
						x=0; y=20000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=6
							role=terminal
						}
					}
					ha:polygon.8 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=22000; }
							ha:line { x1=0; y1=22000; x2=4000; y2=22000; }
							ha:line { x1=4000; y1=22000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=USB_B_180_degree_PTH_universal-v1.rf
					name=USBB
					role=symbol
					spice/omit=yes
				}
			}
			ha:group.7 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAACT;
				x=0; y=8000;
				li:objects {
					ha:line.1 { x1=16000; y1=28000; x2=16000; y2=40000; stroke=wire; }
					ha:line.2 { x1=16000; y1=40000; x2=12000; y2=40000; stroke=wire; }
					ha:line.3 { x1=12000; y1=36000; x2=16000; y2=36000; stroke=wire; }
					ha:line.4 { x1=16000; y1=36000; x2=16000; y2=36000; stroke=junction; }
					ha:line.5 { x1=16000; y1=32000; x2=12000; y2=32000; stroke=wire; }
					ha:line.6 { x1=16000; y1=32000; x2=16000; y2=32000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.11 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAACU;
				x=0; y=8000;
				li:objects {
					ha:line.1 { x1=12000; y1=52000; x2=16000; y2=52000; stroke=wire; }
					ha:line.2 { x1=16000; y1=52000; x2=16000; y2=56000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.13 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAACV;
				x=12000; y=8000;
				li:objects {
					ha:line.1 { x1=52000; y1=56000; x2=52000; y2=48000; stroke=wire; }
					ha:line.2 { x1=52000; y1=52000; x2=80000; y2=52000; stroke=wire; }
					ha:line.3 { x1=52000; y1=52000; x2=52000; y2=52000; stroke=junction; }
					ha:line.4 { x1=80000; y1=52000; x2=80000; y2=48000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.16 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAACW;
				x=12000; y=8000;
				li:objects {
					ha:line.1 { x1=52000; y1=28000; x2=52000; y2=24000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.19 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAACX;
				x=12000; y=8000;
				li:objects {
					ha:line.1 { x1=80000; y1=28000; x2=80000; y2=24000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.22 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAACY;
				x=12000; y=8000;
				li:objects {
					ha:line.1 { x1=80000; y1=8000; x2=80000; y2=4000; stroke=wire; }
					ha:line.2 { x1=80000; y1=4000; x2=52000; y2=4000; stroke=wire; }
					ha:line.4 { x1=52000; y1=0; x2=52000; y2=8000; stroke=wire; }
					ha:line.5 { x1=52000; y1=4000; x2=52000; y2=4000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.26 {
				li:conn {
					/2/7/2
					/2/6/5/1
				}
			}
			ha:connection.27 {
				li:conn {
					/2/7/3
					/2/6/6/1
				}
			}
			ha:connection.28 {
				li:conn {
					/2/7/5
					/2/6/7/1
				}
			}
			ha:connection.29 {
				li:conn {
					/2/11/1
					/2/6/2/1
				}
			}
			ha:group.42 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAACd; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=16000; y=64000;
				li:objects {
					ha:group.1 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAACe; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:connection.43 {
				li:conn {
					/2/42/1/1
					/2/11/2
				}
			}
			ha:group.44 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAACf; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=64000; y=64000;
				li:objects {
					ha:group.1 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAACg; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.46 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAACl; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=16000; y=36000;
				li:objects {
					ha:group.1 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAACm; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:connection.47 {
				li:conn {
					/2/46/1/1
					/2/7/1
				}
			}
			ha:group.48 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAACn; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=64000; y=8000;
				li:objects {
					ha:group.1 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAACo; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.60 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAACp; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=44000; y=52000;
				li:objects {
					ha:group.1 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAACq; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.61 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAACt; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=44000; y=64000;
				li:objects {
					ha:group.1 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAACu; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.62 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAADR; src_uuid=VBZ6rJ6kY0wH4rUapfwAAADO;
				x=36000; y=60000; mirx=1; miry=1;
				li:objects {
					ha:text.1 { x1=-4000; y1=-6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAADS; src_uuid=VBZ6rJ6kY0wH4rUapfwAAADP;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=VBZ6rJ6kY0wH4rUapfwAAADT; src_uuid=VBZ6rJ6kY0wH4rUapfwAAADQ;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:polygon.4 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=6000; }
							ha:line { x1=0; y1=6000; x2=4000; y2=6000; }
							ha:line { x1=4000; y1=6000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=connector(1,2)
					name=EXT5V
					role=symbol
					spice/omit=yes
				}
			}
			ha:group.63 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAADU;
				x=-4000; y=0;
				li:objects {
					ha:line.1 { x1=44000; y1=60000; x2=48000; y2=60000; stroke=wire; }
					ha:line.2 { x1=48000; y1=60000; x2=48000; y2=64000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.66 {
				uuid=VBZ6rJ6kY0wH4rUapfwAAADV;
				x=-4000; y=0;
				li:objects {
					ha:line.1 { x1=44000; y1=56000; x2=48000; y2=56000; stroke=wire; }
					ha:line.2 { x1=48000; y1=56000; x2=48000; y2=52000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.69 {
				li:conn {
					/2/13/1
					/2/2/2/1
				}
			}
			ha:connection.70 {
				li:conn {
					/2/13/4
					/2/3/2/1
				}
			}
			ha:connection.71 {
				li:conn {
					/2/16/1
					/2/2/1/1
				}
			}
			ha:connection.72 {
				li:conn {
					/2/16/1
					/2/4/2/1
				}
			}
			ha:connection.73 {
				li:conn {
					/2/19/1
					/2/3/1/1
				}
			}
			ha:connection.74 {
				li:conn {
					/2/19/1
					/2/5/2/1
				}
			}
			ha:connection.75 {
				li:conn {
					/2/22/1
					/2/5/1/1
				}
			}
			ha:connection.76 {
				li:conn {
					/2/22/4
					/2/4/1/1
				}
			}
			ha:connection.77 {
				li:conn {
					/2/44/1/1
					/2/13/1
				}
			}
			ha:connection.78 {
				li:conn {
					/2/48/1/1
					/2/22/4
				}
			}
			ha:connection.79 {
				li:conn {
					/2/63/1
					/2/62/2/1
				}
			}
			ha:connection.80 {
				li:conn {
					/2/63/2
					/2/61/1/1
				}
			}
			ha:connection.81 {
				li:conn {
					/2/66/1
					/2/62/3/1
				}
			}
			ha:connection.82 {
				li:conn {
					/2/66/2
					/2/60/1/1
				}
			}
		}
		ha:attrib {
			drawing_min_height=70000
			drawing_min_width=114000
			maintainer=<maint. attr>
			page=<page attr>
			print_page=A/4
			title=<please set sheet title attribute>
		}
	}
  li:sch-rnd-conf-v1 {
   ha:overwrite {
    ha:editor {
     grids_idx = 0
     grid = 1.0240 mm
    }
   }
  }
}
