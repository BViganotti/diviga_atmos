cargo fmt

echo "BUILDING"
cross build --release --target=arm-unknown-linux-gnueabihf

echo "PUSHING"
scp -o StrictHostKeyChecking=no target/arm-unknown-linux-gnueabihf/release/readwrite naughty@naughty:~/atmos777
