# Clean generated files
clean:
	rm -f test/*.s test/*.run runtime/our_code.o runtime/libour_code.a

# Generate assembly (.s) from .snek using cargo
test/%.s: test/%.snek src/main.rs
	cargo run -- $< test/$*.s

# Generate the final executable (.run) from assembly
test/%.run: test/%.s runtime/start.rs
	nasm -f macho64 test/$*.s -o runtime/our_code.o
	ar rcs runtime/libour_code.a runtime/our_code.o
	rustc --target x86_64-apple-darwin -L runtime/ -l static=our_code runtime/start.rs -o test/$*.run

