test:
	cargo test

readme:
	echo '# imglife' 		>  README.md
	echo 							  >> README.md
	echo '```' 					>> README.md
	cargo run -- --help >> README.md
	echo '```' 					>> README.md

step-blinkers:
	cargo run -- --alive '#00D1FB' --dead black blinkers.png output.png
	open output.png

step-104p177:
	cargo build --release
	cp 104P177.png output0.png
	for i in `seq 0 177`; do ./target/release/imglife --alive '#00D1FB' --dead black output$i.png output$((i + 1)).png; done
	open output176.png
