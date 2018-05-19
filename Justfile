test:
	cargo test

readme:
	echo '# imglife' 		>  README.md
	echo 							  >> README.md
	echo '```' 					>> README.md
	cargo run -- --help >> README.md
	echo '```' 					>> README.md

step:
	cargo run -- --alive '#00D1FB' --dead black blinkers.png output.png
	open output.png
