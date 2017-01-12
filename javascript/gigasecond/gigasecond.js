"use strict";

var Gigasecond = function(date) {
    this._date = date;
    this._date.setSeconds(this._date.getSeconds() + 1e9);
};

Gigasecond.prototype.date = function() {
    return this._date;
};

module.exports = Gigasecond;
