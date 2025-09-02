# Qapish Web Style Guide

## Structure

The styles for the Qapish web application are organized using SCSS for better maintainability and modularity.

### Directory Structure

```
style/
├── leptonic.scss          # Main stylesheet with imports
└── components/            # Component-specific styles
    ├── landing.scss       # Landing page styles
    ├── packages.scss      # Package grid and cards
    ├── package-detail.scss # Package detail page
    └── buttons.scss       # Buttons and CTAs
```

## Color Variables

All colors are defined as CSS variables in `leptonic.scss`:

- `--qp-ink`: #0f172a (Primary dark color)
- `--qp-ink-100`: #e2e8f0 (Light accent)
- `--qp-cyan`: #06b6d4 (Primary accent color)
- `--qp-success`: #22c55e (Success states)
- `--qp-warning`: #f59e0b (Warning states)
- `--qp-danger`: #e11d48 (Error states)
- `--qp-surface`: #ffffff (Light surface)
- `--qp-surface-dark`: #0b1220 (Dark surface)
- `--qp-text`: #0b1220 (Text on light backgrounds)
- `--qp-text-dark`: #e2e8f0 (Text on dark backgrounds)

## Typography

- **UI Font**: Inter (with system fallbacks)
- **Monospace Font**: JetBrains Mono (for technical content)

## Component Styles

### Landing Page (`landing.scss`)
- Hero section with GPU visualization
- Security features grid
- Footer styling
- Loading and error states

### Package Components (`packages.scss`)
- Package grid layout (responsive)
- Package cards with hover effects
- Popular package badge
- Pricing displays
- Availability indicators
- Hardware specifications

### Package Detail Page (`package-detail.scss`)
- **Navigation**:
  - Grid-based layout for proper alignment
  - Previous/Next links aligned to edges
  - Package counter centered
  - Responsive mobile layout
- Package header with SKU
- Image gallery
- Technical specifications grid
- Pricing and payment policy
- Action buttons

### Buttons (`buttons.scss`)
- Primary CTA buttons
- Secondary variant buttons
- Large size variant
- Navigation links
- Dark mode support

## Responsive Design

All components are responsive with breakpoints at:
- Mobile: < 768px
- Desktop: ≥ 768px

## Dark Mode

The application supports both light and dark modes using:
- CSS `prefers-color-scheme` media queries
- `.dark` class for explicit dark mode

## Navigation Layout

The package detail page navigation uses a CSS Grid layout for proper alignment:

```scss
.package-nav-container {
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    align-items: center;
    gap: 2rem;
}
```

This ensures:
- Previous link aligns to the left
- Counter stays centered
- Next link aligns to the right

## Best Practices

1. **Use CSS Variables**: Always use the defined color variables for consistency
2. **Mobile First**: Design for mobile, then enhance for desktop
3. **Semantic Classes**: Use descriptive class names that indicate purpose
4. **Component Isolation**: Keep component styles in separate files
5. **Nesting**: Use SCSS nesting sparingly (max 3 levels deep)
6. **Transitions**: Add smooth transitions for interactive elements
7. **Accessibility**: Ensure sufficient color contrast and focus states

## Adding New Styles

1. Create a new SCSS file in `components/` directory
2. Import it in `leptonic.scss`
3. Follow the existing naming conventions
4. Add responsive breakpoints as needed
5. Test in both light and dark modes

## Build Process

Styles are compiled automatically during the build process:
```bash
trunk build --release
```

For development with hot reload:
```bash
trunk serve
```

## Maintenance

- Keep specificity low to avoid conflicts
- Document any complex layouts or calculations
- Test across different browsers and devices
- Maintain consistency with the design system