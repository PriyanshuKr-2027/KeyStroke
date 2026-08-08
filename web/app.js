/* ==========================================================================
   Cleo (web.meetcleo.com) Interactive JavaScript Engine
   ========================================================================== */

document.addEventListener('DOMContentLoaded', () => {
  initPhone3DMotion();
  initChatSimulator();
  initModalHandlers();
  initNavbarScroll();
});

/* --------------------------------------------------------------------------
   1. 3D Phone Perspective & Gyroscope/Mouse Motion
   -------------------------------------------------------------------------- */
function initPhone3DMotion() {
  const viewport = document.getElementById('phone-viewport');
  const wrapper = document.getElementById('phone-wrapper');

  if (!viewport || !wrapper) return;

  let mouseX = 0;
  let mouseY = 0;
  let targetRotateX = 0;
  let targetRotateY = 0;
  let currentRotateX = 0;
  let currentRotateY = 0;

  window.addEventListener('mousemove', (e) => {
    const rect = viewport.getBoundingClientRect();
    const centerX = rect.left + rect.width / 2;
    const centerY = rect.top + rect.height / 2;

    mouseX = e.clientX - centerX;
    mouseY = e.clientY - centerY;

    // Calculate rotation angles (max tilt +/- 18deg)
    targetRotateY = (mouseX / (window.innerWidth / 2)) * 20;
    targetRotateX = -(mouseY / (window.innerHeight / 2)) * 20;
  });

  // Smooth lerp loop for 60fps fluid 3D phone tilt
  function renderLoop() {
    currentRotateX += (targetRotateX - currentRotateX) * 0.1;
    currentRotateY += (targetRotateY - currentRotateY) * 0.1;

    wrapper.style.transform = `
      rotateX(${currentRotateX.toFixed(2)}deg) 
      rotateY(${currentRotateY.toFixed(2)}deg) 
      translateZ(20px)
    `;

    requestAnimationFrame(renderLoop);
  }

  renderLoop();
}

/* --------------------------------------------------------------------------
   2. Interactive Chat Simulator (Roast vs. Hype Modes)
   -------------------------------------------------------------------------- */
const chatResponses = {
  spending: {
    user: "Roast my Amazon impulse buys",
    roast: "You bought a $35 banana slicer and an aesthetic desk light at 2 AM. Your cart is a cry for help 💀.",
    hype: "Recognizing your triggers is step one! Let's lock those impulse buys into a 24-hour cooling period and watch your savings grow 🚀!"
  },
  paycheck: {
    user: "Can I buy a festival ticket?",
    roast: "Bestie, your bank balance is currently $42.10. Unless the festival accepts exposure as payment, sit this one out 😭.",
    hype: "You've got $120 extra saved in your festival vault! If you skip 2 dining outs next week, you're 100% good to go 🎉!"
  },
  saving: {
    user: "How do I save $500 this month?",
    roast: "Step 1: Stop buying iced coffee like it's a tax write-off. Step 2: Actually check your bank account before tapping your card.",
    hype: "Easy win! We can auto-set aside $16.50 a day into your smart vault. You won't even feel it leaving your account 💪!"
  },
  coffee: {
    user: "Why am I broke after payday?",
    roast: "Because you treated yourself to 4 dinners, paid 3 subscriptions you don't watch, and forgot rent exists. Math doesn't lie 📉.",
    hype: "No stress! Let's automate your fixed bills on day 1 so your fun budget is crystal clear for the rest of the month 🌟!"
  }
};

let currentMode = 'roast';

function initChatSimulator() {
  const windowContainer = document.getElementById('interactive-chat-window');
  const roastBtn = document.getElementById('roast-mode-btn');
  const hypeBtn = document.getElementById('hype-mode-btn');
  const promptPills = document.querySelectorAll('.prompt-pill');

  if (!windowContainer) return;

  // Set initial state
  renderChatResponse('spending');

  // Mode Switchers
  roastBtn?.addEventListener('click', () => {
    currentMode = 'roast';
    roastBtn.classList.add('active');
    hypeBtn?.classList.remove('active');
    renderChatResponse('spending');
  });

  hypeBtn?.addEventListener('click', () => {
    currentMode = 'hype';
    hypeBtn.classList.add('active');
    roastBtn?.classList.remove('active');
    renderChatResponse('spending');
  });

  // Prompt Pill Click Handlers
  promptPills.forEach((pill) => {
    pill.addEventListener('click', () => {
      const key = pill.getAttribute('data-prompt');
      if (key && chatResponses[key]) {
        renderChatResponse(key);
      }
    });
  });
}

function renderChatResponse(promptKey) {
  const windowContainer = document.getElementById('interactive-chat-window');
  if (!windowContainer) return;

  const data = chatResponses[promptKey] || chatResponses.spending;
  const replyText = currentMode === 'roast' ? data.roast : data.hype;
  const badgeColor = currentMode === 'roast' ? '#ef4444' : 'var(--accent-lime)';

  windowContainer.innerHTML = `
    <div class="chat-bubble user" style="align-self: flex-end;">
      ${data.user}
    </div>
    <div class="chat-bubble cleo" style="align-self: flex-start; border-left: 3px solid ${badgeColor};">
      <div style="display: flex; align-items: center; gap: 6px; font-weight: 700; font-size: 0.75rem; color: ${badgeColor}; margin-bottom: 4px;">
        ${currentMode === 'roast' ? '🔥 ROAST MODE' : '⚡ HYPE MODE'}
      </div>
      ${replyText}
    </div>
  `;
}

/* --------------------------------------------------------------------------
   3. QR Modal Handler
   -------------------------------------------------------------------------- */
function initModalHandlers() {
  const modal = document.getElementById('qr-modal');
  const openBtns = [
    document.getElementById('open-qr-btn'),
    document.getElementById('hero-qr-btn'),
    document.getElementById('hero-download-btn')
  ];
  const closeBtn = document.getElementById('close-qr-btn');

  openBtns.forEach((btn) => {
    btn?.addEventListener('click', () => {
      modal?.classList.add('active');
    });
  });

  closeBtn?.addEventListener('click', () => {
    modal?.classList.remove('active');
  });

  modal?.addEventListener('click', (e) => {
    if (e.target === modal) {
      modal.classList.remove('active');
    }
  });
}

/* --------------------------------------------------------------------------
   4. Navbar Scroll Effect
   -------------------------------------------------------------------------- */
function initNavbarScroll() {
  const navbar = document.getElementById('navbar');
  window.addEventListener('scroll', () => {
    if (window.scrollY > 40) {
      navbar?.classList.add('scrolled');
    } else {
      navbar?.classList.remove('scrolled');
    }
  });
}
