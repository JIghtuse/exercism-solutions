var Bob = function() {};

function is_shouting(input) {
    for (const c of input) {
        if (c != c.toUpperCase()) {
            return false;
        }
    }
    return input.match(/[a-z]/i);
}

Bob.prototype.hey = function(input) {
    if (is_shouting(input)) {
        return 'Whoa, chill out!';
    } else if (input[input.length - 1] == '?') {
        return 'Sure.';
    } else if (!input.trim()) {
        return 'Fine. Be that way!';
    }
    return 'Whatever.';
};

module.exports = Bob;
