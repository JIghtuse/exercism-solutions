var Year = function(year) {
  this.year = year;
};

Year.prototype.isLeap = function() {
  var year = this.year;
  function year_divisible_by(n) {
    return year % n === 0;
  }
  return year_divisible_by(4) && !year_divisible_by(100) || year_divisible_by(400);
};

module.exports = Year;
