import { usePreferencesStore } from "../../stores";
import { revealLabel } from "../../utils/platform";

interface Props {
  isFavorite: boolean;
  isInBasket: boolean;
  onFavorite: () => void;
  onToggleBasket: () => void;
  onReveal: () => void;
}

export default function QuickActions({
  isFavorite,
  isInBasket,
  onFavorite,
  onToggleBasket,
  onReveal,
}: Props) {
  const showTooltips = usePreferencesStore((s) => s.interaction.show_tooltips);

  const btnStyle: React.CSSProperties = {
    width: 28,
    height: 28,
    borderRadius: 8,
    border: "none",
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    cursor: "pointer",
    fontSize: 11,
    background: "rgba(0,0,0,0.5)",
    backdropFilter: "blur(8px)",
    color: "var(--evoury-text-dim)",
    transition: "all 0.15s ease",
  };

  return (
    <div style={{ display: "flex", gap: 4, alignItems: "center" }}>
      <button
        style={{ ...btnStyle, color: isFavorite ? "#f59e0b" : undefined }}
        onClick={(e) => {
          e.stopPropagation();
          onFavorite();
        }}
        title={showTooltips ? "Toggle Favorite" : undefined}
        onMouseEnter={(e) => {
          e.currentTarget.style.background = "rgba(255,255,255,0.15)";
          e.currentTarget.style.color = "#f59e0b";
        }}
        onMouseLeave={(e) => {
          e.currentTarget.style.background = "rgba(0,0,0,0.5)";
          e.currentTarget.style.color = isFavorite
            ? "#f59e0b"
            : "var(--evoury-text-dim)";
        }}
      >
        <i className={`fas fa-${isFavorite ? "star" : "star"}`} />
      </button>
      <button
        style={btnStyle}
        onClick={(e) => {
          e.stopPropagation();
          onToggleBasket();
        }}
        title={
          showTooltips
            ? isInBasket
              ? "Remove from Basket"
              : "Add to Basket"
            : undefined
        }
        onMouseEnter={(e) => {
          e.currentTarget.style.background = "rgba(255,255,255,0.15)";
        }}
        onMouseLeave={(e) => {
          e.currentTarget.style.background = "rgba(0,0,0,0.5)";
        }}
      >
        <i
          className={`fas fa-${isInBasket ? "shopping-basket" : "basket-shopping"}`}
          style={{ color: isInBasket ? "#6366f1" : undefined }}
        />
      </button>
      <button
        style={btnStyle}
        onClick={(e) => {
          e.stopPropagation();
          onReveal();
        }}
        title={showTooltips ? revealLabel() : undefined}
        onMouseEnter={(e) => {
          e.currentTarget.style.background = "rgba(255,255,255,0.15)";
        }}
        onMouseLeave={(e) => {
          e.currentTarget.style.background = "rgba(0,0,0,0.5)";
        }}
      >
        <i className="fas fa-folder-open" />
      </button>
    </div>
  );
}
