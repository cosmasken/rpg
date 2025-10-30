/**
 * Mathematical utility functions for the RPG game.
 * @namespace
 */
export const math = (function() {
  return {
    /**
     * Generate a random float in the range [a, b)
     * @param {number} a - Lower bound (inclusive)
     * @param {number} b - Upper bound (exclusive)
     * @returns {number} Random float between a and b
     */
    rand_range: function(a, b) {
      return Math.random() * (b - a) + a;
    },

    /**
     * Generate a random number with a normalish distribution between -1 and 1
     * @returns {number} Random number with normalish distribution
     */
    rand_normalish: function() {
      const r = Math.random() + Math.random() + Math.random() + Math.random();
      return (r / 4.0) * 2.0 - 1;
    },

    /**
     * Generate a random integer in the range [a, b] inclusive
     * @param {number} a - Lower bound (inclusive)
     * @param {number} b - Upper bound (inclusive)
     * @returns {number} Random integer between a and b
     */
    rand_int: function(a, b) {
      return Math.round(Math.random() * (b - a) + a);
    },

    /**
     * Linear interpolation between a and b based on x
     * @param {number} x - Interpolation factor [0, 1]
     * @param {number} a - Start value
     * @param {number} b - End value
     * @returns {number} Interpolated value
     */
    lerp: function(x, a, b) {
      return x * (b - a) + a;
    },

    /**
     * Smoothstep function for smooth interpolation
     * @param {number} x - Interpolation factor [0, 1]
     * @param {number} a - Start value
     * @param {number} b - End value
     * @returns {number} Smoothly interpolated value
     */
    smoothstep: function(x, a, b) {
      x = x * x * (3.0 - 2.0 * x);
      return x * (b - a) + a;
    },

    /**
     * Smootherstep function for even smoother interpolation
     * @param {number} x - Interpolation factor [0, 1]
     * @param {number} a - Start value
     * @param {number} b - End value
     * @returns {number} Very smoothly interpolated value
     */
    smootherstep: function(x, a, b) {
      x = x * x * x * (x * (x * 6 - 15) + 10);
      return x * (b - a) + a;
    },

    /**
     * Clamp a value between min and max
     * @param {number} x - Value to clamp
     * @param {number} a - Minimum value
     * @param {number} b - Maximum value
     * @returns {number} Clamped value
     */
    clamp: function(x, a, b) {
      return Math.min(Math.max(x, a), b);
    },

    /**
     * Saturate a value to [0, 1] range
     * @param {number} x - Value to saturate
     * @returns {number} Saturated value in [0, 1] range
     */
    sat: function(x) {
      return Math.min(Math.max(x, 0.0), 1.0);
    },

    /**
     * Check if a value is in the range [a, b] inclusive
     * @param {number} x - Value to check
     * @param {number} a - Lower bound
     * @param {number} b - Upper bound
     * @returns {boolean} True if x is in range [a, b]
     */
    in_range: (x, a, b) => {
      return x >= a && x <= b;
    },
  };
})();
