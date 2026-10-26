module.exports = {
  content: ['./src/ui.rs'],
  darkMode: 'class',
  theme: {
    extend: {
      fontFamily: { sans: ['Inter', 'sans-serif'] },
      colors: {
        seerr: {
          bg: '#080c14', card: '#111827', border: '#1f2937',
          accent: '#6366f1', accentHover: '#4f46e5'
        }
      }
    }
  }
};
