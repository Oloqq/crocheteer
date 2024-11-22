export function setupTooltip(element: HTMLElement, textShown: () => string) {
  const tooltip = document.createElement("div");
  tooltip.className = "tooltip";
  document.body.appendChild(tooltip);

  const showTooltip = (event: MouseEvent) => {
    tooltip.style.opacity = "1";
    tooltip.textContent = textShown();
    positionTooltip(event);
  };

  const moveTooltip = (event: MouseEvent) => {
    positionTooltip(event);
  };

  const hideTooltip = () => {
    tooltip.style.opacity = "0";
  };

  // Function to position the tooltip near the cursor
  const positionTooltip = (event: MouseEvent) => {
    const tooltipRect = tooltip.getBoundingClientRect();
    const offset = 10; // Distance between cursor and tooltip

    let x = event.pageX + offset;
    let y = event.pageY + offset;

    // Prevent tooltip from going off the right edge
    if (x + tooltipRect.width > window.innerWidth) {
      x = event.pageX - tooltipRect.width - offset;
    }

    // Prevent tooltip from going off the bottom edge
    if (y + tooltipRect.height > window.innerHeight) {
      y = event.pageY - tooltipRect.height - offset;
    }

    tooltip.style.left = `${x}px`;
    tooltip.style.top = `${y}px`;
  };

  // Attach event listeners to the status button
  element.addEventListener("mouseenter", showTooltip);
  element.addEventListener("mousemove", moveTooltip);
  element.addEventListener("mouseleave", hideTooltip);
}
