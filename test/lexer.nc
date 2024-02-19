# hello world!
this code is written in nightcode #

var array = [1, 2, 3, 4, 5];

fun main()
    while true
        var value = pow(rand::range(-10, 10), 2);
        if value >= 10 do array.push(value);

        console::print("rand value was ${math::sqrt(value)}");
        console::print(array.length);

        thread::wait_seconds(1);
    end

    bind w_main = while true
        var rand = rand::range(5, 60);
        for el in array
            if el == rand do break w_main;
        end
    end

    array.clear();
end

fun pow(f32 value, i32 power): f32
    var output = value;
    for iter::repeat(power) do output *= value;

    return output;
end