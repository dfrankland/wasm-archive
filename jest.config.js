module.exports = {
  testURL: 'http://localhost',
  moduleFileExtensions: ['js', 'ts', 'wasm'],
  testMatch: ['**/tests/**/*.ts'],
  collectCoverage: true,
  collectCoverageFrom: ['pkg/**/*.js'],
  coverageThreshold: {
    global: {
      statements: 85,
      branches: 85,
      functions: 85,
      lines: 85,
    },
  },
};
