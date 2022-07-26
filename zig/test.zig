
const Foo = struct {
    bar: [5]u8,
};

fn GenStruct(comptime T: type) type {
    return struct {
        pub fn some_method(some_value: T) T {
            return some_value;
        }
    };
}

const I32GenStruct = GenStruct(i32);
const FooGenStruct = GenStruct(Foo);

