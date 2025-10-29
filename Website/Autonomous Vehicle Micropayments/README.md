# AutonomousFlow Website Template

A modern, engaging landing page for the AutonomousFlow autonomous vehicle payments project.

## 🎨 Design Features

- **Modern Dark Theme**: Professional dark design with gradient accents
- **Responsive Layout**: Mobile-first design that works on all devices
- **Smooth Animations**: Engaging scroll animations and interactive elements
- **Performance Optimized**: Fast loading with optimized assets

## 🚀 Getting Started

### Prerequisites
- Web server (local or remote)
- Modern web browser

### Installation

1. **Clone or download the website files**
   ```bash
   git clone [repository-url]
   cd autonomous-vehicle-payments/website
   ```

2. **Serve the files**
   
   **Option A: Python Simple Server**
   ```bash
   python -m http.server 8000
   ```
   
   **Option B: Node.js Live Server**
   ```bash
   npx live-server
   ```
   
   **Option C: PHP Built-in Server**
   ```bash
   php -S localhost:8000
   ```

3. **Open in browser**
   ```
   http://localhost:8000
   ```

## 📁 File Structure

```
website/
├── index.html          # Main landing page
├── styles.css          # All styling and animations
├── script.js           # Interactive functionality
└── README.md          # This file
```

## 🎯 Sections Overview

### 1. Navigation
- Fixed header with smooth scrolling
- Mobile-responsive hamburger menu
- Call-to-action button

### 2. Hero Section
- Compelling headline with gradient text
- Animated floating card showing vehicle status
- Key statistics with counter animations
- Dual call-to-action buttons

### 3. Features Section
- 6 key features with icons and descriptions
- Hover effects and animations
- Grid layout adapts to screen size

### 4. How It Works
- 4-step process visualization
- Clear flow with arrows and icons
- Step-by-step explanation

### 5. Technology Stack
- Showcases core technologies
- Solana, Anchor, IoT, AI integration
- Card-based layout with hover effects

### 6. Call-to-Action
- Gradient background section
- Multiple action buttons
- Compelling messaging

### 7. Footer
- Company information and links
- Social media integration
- Legal and resource links

## 🎨 Customization

### Colors (CSS Variables)
```css
:root {
    --primary-color: #9945FF;      /* Solana purple */
    --secondary-color: #14F195;    /* Solana green */
    --accent-color: #FF6B6B;       /* Red accent */
    --dark-bg: #0A0A0B;           /* Main background */
    --dark-card: #1A1B23;         /* Card backgrounds */
}
```

### Typography
- **Font**: Inter (Google Fonts)
- **Weights**: 300, 400, 500, 600, 700, 800

### Animations
- **CSS Animations**: Floating elements, progress bars, gradients
- **JavaScript Animations**: Counter animations, scroll effects
- **Intersection Observer**: Trigger animations on scroll

## 📱 Responsive Breakpoints

- **Desktop**: 1200px+
- **Tablet**: 768px - 1199px
- **Mobile**: 320px - 767px

## ⚡ Performance Features

- **Optimized Loading**: Minimal external dependencies
- **Lazy Loading**: Images and animations load when needed
- **Smooth Animations**: 60fps animations with transform properties
- **Compressed Assets**: Optimized file sizes

## 🔧 Technical Features

### JavaScript Functionality
- Mobile navigation toggle
- Smooth scrolling navigation
- Counter animations
- Intersection Observer for scroll animations
- Dynamic background effects
- Accessibility enhancements
- Performance monitoring

### CSS Features
- CSS Grid and Flexbox layouts
- Custom properties (CSS variables)
- Modern animations and transitions
- Responsive design patterns
- Dark theme optimization

### Accessibility
- Keyboard navigation support
- Focus indicators
- Semantic HTML structure
- Alt text for images
- Screen reader friendly

## 🚀 Deployment Options

### 1. Static Hosting
- **Netlify**: Drag and drop deployment
- **Vercel**: Git integration
- **GitHub Pages**: Free hosting
- **AWS S3**: Static website hosting

### 2. CDN Integration
- **Cloudflare**: Performance and security
- **AWS CloudFront**: Global distribution
- **Google Cloud CDN**: Fast delivery

### 3. Custom Domain
```bash
# Add CNAME record pointing to your hosting provider
# Example for Netlify:
CNAME: your-domain.com -> your-site.netlify.app
```

## 🔮 Future Enhancements

### Phase 1: Dynamic Content
- [ ] Connect to real vehicle data APIs
- [ ] Live delivery tracking
- [ ] Real-time statistics

### Phase 2: User Interaction
- [ ] Contact form with backend
- [ ] Newsletter signup
- [ ] User dashboard integration

### Phase 3: E-commerce Integration
- [ ] Payment gateway integration
- [ ] Subscription management
- [ ] Customer portal

### Phase 4: Advanced Features
- [ ] Multi-language support
- [ ] Advanced animations
- [ ] Progressive Web App (PWA)

## 📊 Analytics Integration

Add tracking codes to `index.html`:

```html
<!-- Google Analytics -->
<script async src="https://www.googletagmanager.com/gtag/js?id=GA_TRACKING_ID"></script>

<!-- Facebook Pixel -->
<script>
  !function(f,b,e,v,n,t,s) { /* Facebook Pixel Code */ }
</script>
```

## 🎨 Brand Guidelines

### Logo Usage
- Primary logo: Icon + text combination
- Use on dark backgrounds
- Maintain clear space around logo

### Color Palette
- **Primary**: Solana purple (#9945FF)
- **Secondary**: Solana green (#14F195)
- **Accent**: Coral red (#FF6B6B)
- **Neutral**: Dark backgrounds with white text

### Typography Hierarchy
- **H1**: 3.5rem (56px) - Hero titles
- **H2**: 2.5rem (40px) - Section titles
- **H3**: 1.5rem (24px) - Card titles
- **Body**: 1rem (16px) - Regular text
- **Small**: 0.9rem (14px) - Captions

## 🛠️ Development Setup

### Local Development
```bash
# Install dependencies (if using build tools)
npm install

# Start development server
npm run dev

# Build for production
npm run build
```

### Code Quality
- **HTML Validation**: W3C Markup Validator
- **CSS Linting**: Stylelint
- **JavaScript Linting**: ESLint
- **Accessibility Testing**: axe-core

## 📞 Support

- **Documentation**: Internal project docs
- **Issues**: Create GitHub issues for bugs
- **Features**: Submit feature requests
- **Contact**: [your-email@domain.com]

## 📄 License

This website template is part of the AutonomousFlow project.
Licensed under MIT License - see main project LICENSE file.

---

**Built with modern web technologies for the future of autonomous delivery payments.**

🚀 Ready to deploy and customize for your needs!
