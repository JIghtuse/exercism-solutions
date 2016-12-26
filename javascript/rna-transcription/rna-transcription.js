class DnaTranscriber {
  toRna(dna) {
    const rnaMap = new Map([
      ['C', 'G'],
      ['G', 'C'],
      ['T', 'A'],
      ['A', 'U'],
    ]);
    return Array.prototype.map.call(dna, c => rnaMap.get(c)).join('');
  }
}

module.exports = DnaTranscriber;
