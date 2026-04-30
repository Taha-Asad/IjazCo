import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../contexts/AuthContext";
import { useToast } from "../contexts/ToastContext";
import "./LoginPage.css";

// ── Inline SVG icons extracted directly from Figma ──────────────────────────

const AdminIcon = () => (
  <svg width="75" height="75" viewBox="0 0 75 75" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect width="75" height="75" rx="12.4805" fill="#ECFDF5"/>
    <path d="M46.6055 46.6211C47.3656 46.6211 48.0117 46.3551 48.5437 45.823C49.0758 45.2909 49.3418 44.6449 49.3418 43.8848C49.3418 43.1247 49.0758 42.4786 48.5437 41.9465C48.0117 41.4145 47.3656 41.1484 46.6055 41.1484C45.8454 41.1484 45.1993 41.4145 44.6673 41.9465C44.1352 42.4786 43.8692 43.1247 43.8692 43.8848C43.8692 44.6449 44.1352 45.2909 44.6673 45.823C45.1993 46.3551 45.8454 46.6211 46.6055 46.6211ZM46.6055 52.0938C47.548 52.0938 48.4145 51.8733 49.205 51.4325C49.9955 50.9916 50.634 50.4063 51.1204 49.6767C50.4516 49.2814 49.7371 48.9774 48.977 48.7646C48.2169 48.5517 47.4264 48.4453 46.6055 48.4453C45.7846 48.4453 44.9941 48.5517 44.234 48.7646C43.4739 48.9774 42.7594 49.2814 42.0906 49.6767C42.577 50.4063 43.2155 50.9916 44.006 51.4325C44.7965 51.8733 45.663 52.0938 46.6055 52.0938ZM37.4844 55.7422C33.2583 54.6781 29.7695 52.2534 27.018 48.4681C24.2664 44.6829 22.8907 40.4796 22.8907 35.8582V24.7305L37.4844 19.2578L52.0782 24.7305V35.0829C51.5005 34.8397 50.9076 34.6193 50.2995 34.4216C49.6915 34.224 49.0682 34.0796 48.4297 33.9884V27.2844L37.4844 23.1799L26.5391 27.2844V35.8582C26.5391 37.2872 26.7291 38.7161 27.1092 40.1451C27.4892 41.5741 28.0213 42.9347 28.7054 44.2268C29.3894 45.519 30.2179 46.7123 31.1909 47.8068C32.1638 48.9014 33.2431 49.8135 34.4288 50.5432C34.7633 51.5161 35.2041 52.4434 35.7514 53.3251C36.2987 54.2068 36.9219 54.9973 37.6212 55.6966C37.5908 55.6966 37.568 55.7042 37.5528 55.7194C37.5376 55.7346 37.5148 55.7422 37.4844 55.7422ZM46.6055 55.7422C44.082 55.7422 41.9309 54.8529 40.1523 53.0743C38.3737 51.2957 37.4844 49.1446 37.4844 46.6211C37.4844 44.0976 38.3737 41.9465 40.1523 40.1679C41.9309 38.3893 44.082 37.5 46.6055 37.5C49.129 37.5 51.2801 38.3893 53.0587 40.1679C54.8373 41.9465 55.7266 44.0976 55.7266 46.6211C55.7266 49.1446 54.8373 51.2957 53.0587 53.0743C51.2801 54.8529 49.129 55.7422 46.6055 55.7422Z" fill="#059669"/>
  </svg>
);

const AdminArrow = () => (
  <svg width="29" height="35" viewBox="0 0 29 35" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path d="M19.2997 18.6932H4.77275V16.3068H19.2997L12.6179 9.62502L14.3182 7.95457L23.8637 17.5L14.3182 27.0455L12.6179 25.375L19.2997 18.6932Z" fill="#059669"/>
  </svg>
);

const SalesIcon = () => (
  <svg width="75" height="75" viewBox="0 0 75 75" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect width="75" height="75" rx="12.4805" fill="#EFF6FF"/>
    <path d="M28.3633 30.2031C27.36 30.2031 26.501 29.8459 25.7866 29.1314C25.0721 28.4169 24.7148 27.558 24.7148 26.5547V22.9063C24.7148 21.9029 25.0721 21.044 25.7866 20.3295C26.501 19.6151 27.36 19.2578 28.3633 19.2578H46.6055C47.6088 19.2578 48.4677 19.6151 49.1822 20.3295C49.8967 21.044 50.2539 21.9029 50.2539 22.9063V26.5547C50.2539 27.558 49.8967 28.4169 49.1822 29.1314C48.4677 29.8459 47.6088 30.2031 46.6055 30.2031H28.3633ZM28.3633 26.5547H46.6055V22.9063H28.3633V26.5547ZM22.8906 55.7422C21.8873 55.7422 21.0284 55.3849 20.3139 54.6705C19.5994 53.956 19.2422 53.0971 19.2422 52.0938V50.2695H55.7266V52.0938C55.7266 53.0971 55.3693 53.956 54.6548 54.6705C53.9403 55.3849 53.0814 55.7422 52.0781 55.7422H22.8906ZM19.2422 48.4453L25.5813 34.1708C25.8854 33.5019 26.3414 32.9775 26.9495 32.5974C27.5576 32.2174 28.2113 32.0273 28.9105 32.0273H46.0582C46.7575 32.0273 47.4112 32.2174 48.0192 32.5974C48.6273 32.9775 49.0834 33.5019 49.3874 34.1708L55.7266 48.4453H19.2422ZM31.0996 44.7969H32.9238C33.167 44.7969 33.3799 44.7057 33.5623 44.5232C33.7447 44.3408 33.8359 44.128 33.8359 43.8848C33.8359 43.6415 33.7447 43.4287 33.5623 43.2463C33.3799 43.0639 33.167 42.9727 32.9238 42.9727H31.0996C30.8564 42.9727 30.6435 43.0639 30.4611 43.2463C30.2787 43.4287 30.1875 43.6415 30.1875 43.8848C30.1875 44.128 30.2787 44.3408 30.4611 44.5232C30.6435 44.7057 30.8564 44.7969 31.0996 44.7969ZM36.5723 41.1484H38.3965C38.6397 41.1484 38.8525 41.0572 39.0349 40.8748C39.2174 40.6924 39.3086 40.4796 39.3086 40.2363C39.3086 39.9931 39.2174 39.7803 39.0349 39.5979C38.8525 39.4154 38.6397 39.3242 38.3965 39.3242H36.5723C36.329 39.3242 36.1162 39.4154 35.9338 39.5979C35.7514 39.7803 35.6601 39.9931 35.6601 40.2363C35.6601 40.4796 35.7514 40.6924 35.9338 40.8748C36.1162 41.0572 36.329 41.1484 36.5723 41.1484ZM42.0449 37.5H43.8691C44.1124 37.5 44.3252 37.4088 44.5076 37.2264C44.69 37.0439 44.7812 36.8311 44.7812 36.5879C44.7812 36.3447 44.69 36.1318 44.5076 35.9494C44.3252 35.767 44.1124 35.6758 43.8691 35.6758H42.0449C41.8017 35.6758 41.5889 35.767 41.4064 35.9494C41.224 36.1318 41.1328 36.3447 41.1328 36.5879C41.1328 36.8311 41.224 37.0439 41.4064 37.2264C41.5889 37.4088 41.8017 37.5 42.0449 37.5Z" fill="#2563EB"/>
  </svg>
);

const SalesArrow = () => (
  <svg width="29" height="35" viewBox="0 0 29 35" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path d="M19.2997 18.6932H4.77271V16.3068H19.2997L12.6179 9.62502L14.3182 7.95457L23.8636 17.5L14.3182 27.0455L12.6179 25.375L19.2997 18.6932Z" fill="#2563EB"/>
  </svg>
);

const InventoryIcon = () => (
  <svg width="75" height="75" viewBox="0 0 75 75" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect width="75" height="75" rx="12.4805" fill="#FFFBEB"/>
    <path d="M24.7148 55.7422C23.7115 55.7422 22.8526 55.3849 22.1381 54.6704C21.4236 53.956 21.0664 53.0971 21.0664 52.0937V31.5257C20.5191 31.1912 20.0783 30.758 19.7438 30.2259C19.4094 29.6938 19.2422 29.0782 19.2422 28.3789V22.9062C19.2422 21.9029 19.5994 21.044 20.3139 20.3295C21.0284 19.615 21.8873 19.2578 22.8906 19.2578H52.0781C53.0814 19.2578 53.9403 19.615 54.6548 20.3295C55.3693 21.044 55.7266 21.9029 55.7266 22.9062V28.3789C55.7266 29.0782 55.5593 29.6938 55.2249 30.2259C54.8904 30.758 54.4496 31.1912 53.9023 31.5257V52.0937C53.9023 53.0971 53.5451 53.956 52.8306 54.6704C52.1161 55.3849 51.2572 55.7422 50.2539 55.7422H24.7148ZM24.7148 32.0273V52.0937H50.2539V32.0273H24.7148ZM22.8906 28.3789H52.0781V22.9062H22.8906V28.3789ZM32.0117 41.1484H42.957V37.5H32.0117V41.1484Z" fill="#D97706"/>
  </svg>
);

const InventoryArrow = () => (
  <svg width="29" height="35" viewBox="0 0 29 35" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path d="M19.2997 18.6932H4.77275V16.3068H19.2997L12.6179 9.625L14.3182 7.95455L23.8637 17.5L14.3182 27.0455L12.6179 25.375L19.2997 18.6932Z" fill="#D97706"/>
  </svg>
);

// ── Card data matching Figma design ─────────────────────────────────────────

interface ActiveCard {
  id: string;
  title: string;
  description: string;
  cta: string;
  gradientClass: string;
  ctaColor: string;
  Icon: React.FC;
  Arrow: React.FC;
}

const ACTIVE_CARDS: ActiveCard[] = [
  {
    id: "admin-path",
    title: "Admin Path",
    description: "User management, system\nconfiguration, and audit logs.",
    cta: "Enter Workspace",
    gradientClass: "lp-gradient--emerald",
    ctaColor: "#059669",
    Icon: AdminIcon,
    Arrow: AdminArrow,
  },
  {
    id: "sales-path",
    title: "Sales Path",
    description: "Transaction processing, patient\nbilling, and daily registers.",
    cta: "Open Terminal",
    gradientClass: "lp-gradient--blue",
    ctaColor: "#2563EB",
    Icon: SalesIcon,
    Arrow: SalesArrow,
  },
  {
    id: "inventory-path",
    title: "Inventory Path",
    description: "Stock tracking, procurement orders,\nand supplier management.",
    cta: "Manage Stock",
    gradientClass: "lp-gradient--amber",
    ctaColor: "#D97706",
    Icon: InventoryIcon,
    Arrow: InventoryArrow,
  },
];

// ── Component ────────────────────────────────────────────────────────────────

export default function LoginPage() {
  const [showLogin, setShowLogin] = useState(false);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);
  const { login } = useAuth();
  const toast = useToast();
  const navigate = useNavigate();

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setLoading(true);
    try {
      await login({ username, password });
      navigate("/dashboard");
    } catch (err: any) {
      const message = err.message || "Login failed. Please check your credentials.";
      setError(message);
      toast.error(message, "Authentication Error");
    } finally {
      setLoading(false);
    }
  };

  const handleCardClick = () => setShowLogin(true);
  const handleImportsClick = () => setShowLogin(true);

  return (
    <main className="lp-root" id="login-page">
      {/* ── Page header ───────────────────────────────────── */}
      <div className="lp-header">
        <h1 className="lp-title">Choose Your Workspace</h1>
        <p className="lp-subtitle">Select a path to get started</p>
      </div>

      {/* ── 2 × 2 card grid ───────────────────────────────── */}
      <div className="lp-grid">

        {/* Active cards */}
        {ACTIVE_CARDS.map((card) => (
          <button
            key={card.id}
            id={card.id}
            className="lp-card lp-card--active"
            onClick={handleCardClick}
            type="button"
            aria-label={`Open ${card.title}`}
          >
            {/* Gradient accent bar at the top */}
            <span className={`lp-card__bar ${card.gradientClass}`} aria-hidden="true" />

            <div className="lp-card__body">
              {/* Icon */}
              <div className="lp-card__icon-wrap">
                <card.Icon />
              </div>

              {/* Title */}
              <h2 className="lp-card__title">{card.title}</h2>

              {/* Description */}
              <p className="lp-card__desc">
                {card.description.split("\n").map((line, i) => (
                  <span key={i}>{line}{i === 0 && <br />}</span>
                ))}
              </p>

              {/* CTA row */}
              <span className="lp-card__cta" style={{ color: card.ctaColor }}>
                {card.cta}
                <span className="lp-card__arrow">
                  <card.Arrow />
                </span>
              </span>
            </div>
          </button>
        ))}

        {/* Imports card */}
        <button
          id="imports-path"
          className="lp-card lp-card--active"
          type="button"
          onClick={handleImportsClick}
          aria-label="Open Imports Path"
        >
          <div className="lp-card__body">
            {/* Imports background placeholder (slate) */}
            <div className="lp-card__icon-wrap lp-card__icon-wrap--slate" />

            {/* Title */}
            <h2 className="lp-card__title">Imports Path</h2>

            {/* Description */}
            <p className="lp-card__desc">
              Bulk data migration and external<br />system integrations.
            </p>

            <span className="lp-card__cta" style={{ color: "#475569" }}>
              Open Imports
              <span className="lp-card__arrow">
                <SalesArrow />
              </span>
            </span>
          </div>
        </button>

      </div>

      {/* ── Login modal ───────────────────────────────────── */}
      {showLogin && (
        <div
          className="lp-overlay"
          role="dialog"
          aria-modal="true"
          aria-labelledby="login-modal-title"
          onClick={() => setShowLogin(false)}
        >
          <div
            className="lp-modal"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="lp-modal__header">
              <h2 id="login-modal-title" className="lp-modal__title">Sign In</h2>
              <p className="lp-modal__subtitle">Enter your credentials to continue</p>
            </div>

            <form onSubmit={handleLogin} className="lp-modal__form" noValidate>
              {error && (
                <div className="lp-modal__error" role="alert">
                  <svg className="lp-modal__error-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                  </svg>
                  {error}
                </div>
              )}

              <div className="lp-modal__field">
                <label htmlFor="login-username" className="lp-modal__label">
                  Username or Email
                </label>
                <input
                  id="login-username"
                  type="text"
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                  placeholder="Enter your username"
                  required
                  autoComplete="username"
                  className="lp-modal__input"
                />
              </div>

              <div className="lp-modal__field">
                <label htmlFor="login-password" className="lp-modal__label">
                  Password
                </label>
                <input
                  id="login-password"
                  type="password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="Enter your password"
                  required
                  autoComplete="current-password"
                  className="lp-modal__input"
                />
              </div>

              <button
                id="login-submit"
                type="submit"
                disabled={loading || !username || !password}
                className="lp-modal__submit"
              >
                {loading ? "Signing in…" : "Sign In"}
              </button>

              <button
                id="login-back"
                type="button"
                onClick={() => setShowLogin(false)}
                className="lp-modal__back"
              >
                Back to selection
              </button>
            </form>

            <p className="lp-modal__hint">Demo: admin / admin123</p>
          </div>
        </div>
      )}
    </main>
  );
}
