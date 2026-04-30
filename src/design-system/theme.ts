// Theme configuration for consistent ERP UI design
// This defines the visual language for the entire application

export const theme = {
  colors: {
    primary: {
      light: '#818cf8',
      DEFAULT: '#6366f1',
      dark: '#4f46e5',
      darker: '#4338ca',
    },
    gray: {
      50: '#f9fafb',
      100: '#f3f4f6',
      200: '#e5e7eb',
      300: '#d1d5db',
      400: '#9ca3af',
      500: '#6b7280',
      600: '#4b5563',
      700: '#374151',
      800: '#1f2937',
      900: '#111827',
    },
    success: '#10b981',
    warning: '#f59e0b',
    danger: '#ef4444',
    info: '#3b82f6',
  },
  
  gradients: {
    primary: 'bg-gradient-to-br from-indigo-600 via-indigo-700 to-purple-700',
    surface: 'bg-gradient-to-br from-gray-50 to-gray-100/50',
    card: 'bg-gradient-to-br from-white to-gray-50/30',
  },

  shadows: {
    sm: '0 1px 2px rgba(0,0,0,0.04), 0 1px 3px rgba(0,0,0,0.06)',
    md: '0 4px 6px -1px rgba(0,0,0,0.07), 0 2px 4px -1px rgba(0,0,0,0.04)',
    lg: '0 10px 15px -3px rgba(0,0,0,0.08), 0 4px 6px -2px rgba(0,0,0,0.03)',
    xl: '0 20px 25px -5px rgba(0,0,0,0.1), 0 10px 10px -5px rgba(0,0,0,0.04)',
    glow: '0 0 20px rgba(99, 102, 241, 0.15)',
  },

  animations: {
    'fade-in': 'fadeIn 0.3s ease-in-out',
    'slide-up': 'slideUp 0.3s ease-out',
    'scale-in': 'scaleIn 0.2s ease-out',
  },
};

// CSS-in-JS styles for complex components
export const styles = {
  glassmorphism: {
    background: 'rgba(255, 255, 255, 0.7)',
    backdropFilter: 'blur(10px)',
    border: '1px solid rgba(255, 255, 255, 0.2)',
  },
  
  cardHover: {
    transition: 'all 0.2s ease-in-out',
    '&:hover': {
      transform: 'translateY(-2px)',
      boxShadow: theme.shadows.glow,
    },
  },

  shimmer: {
    background: 'linear-gradient(90deg, #f3f4f6 0%, #ffffff 50%, #f3f4f6 100%)',
    backgroundSize: '200% 100%',
    animation: 'shimmer 2s infinite',
  },
};

// Keyframe animations to add to index.css
export const keyframes = `
@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes slideUp {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes scaleIn {
  from { opacity: 0; transform: scale(0.95); }
  to { opacity: 1; transform: scale(1); }
}

@keyframes shimmer {
  0% { background-position: 200% 0; }
  100% { background-position: -200% 0; }
}

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-10px); }
}
`;
