class Hamming {
  compute(strand_a, strand_b) {
    if (strand_a.length != strand_b.length) {
      throw new Error('DNA strands must be of equal length.');
    }
    let count = 0;
    for (let i = 0; i < strand_a.length; i++) {
      if (strand_a[i] != strand_b[i]) {
        count++;
      }
    }
    return count;
  }
}

module.exports = Hamming;
