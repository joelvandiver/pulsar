# PULSAR

Every rustacean needs a shell!

Your shell scripts deserve a type system.

<!-- https://claude.ai/code/artifact/ec7e5188-d07a-404a-bf2d-dccb3b41299d?via=auto_preview -->

<svg width="0" height="0" style="position:absolute" aria-hidden="true">
  <symbol id="pulsar-mark" viewBox="0 0 256 256">
    <defs>
      <linearGradient id="beamN" x1="128" y1="128" x2="128" y2="34" gradientUnits="userSpaceOnUse">
        <stop offset="0" stop-color="#FFB86B" stop-opacity=".8"/>
        <stop offset="1" stop-color="#FFB86B" stop-opacity="0"/>
      </linearGradient>
      <linearGradient id="beamS" x1="128" y1="128" x2="128" y2="222" gradientUnits="userSpaceOnUse">
        <stop offset="0" stop-color="#FFB86B" stop-opacity=".8"/>
        <stop offset="1" stop-color="#FFB86B" stop-opacity="0"/>
      </linearGradient>
      <radialGradient id="starGlow" cx=".5" cy=".5" r=".5">
        <stop offset="0" stop-color="#FFF7EE" stop-opacity=".95"/>
        <stop offset=".5" stop-color="#FFD9A8" stop-opacity=".55"/>
        <stop offset="1" stop-color="#FFB86B" stop-opacity="0"/>
      </radialGradient>
    </defs>
    <!-- lighthouse beams -->
    <path d="M128 128 L116 34 L140 34 Z" fill="url(#beamN)"/>
    <path d="M128 128 L116 222 L140 222 Z" fill="url(#beamS)"/>
    <!-- pulse arcs -->
    <g fill="none" stroke="#F74C00" stroke-linecap="round">
      <path class="arc-a" style="--o:.85" d="M106 81 A52 52 0 0 1 150 81" stroke-width="7" opacity=".85"/>
      <path class="arc-b" style="--o:.55" d="M97 61 A74 74 0 0 1 159 61" stroke-width="6" opacity=".55"/>
      <path class="arc-c" style="--o:.3" d="M87 41 A96 96 0 0 1 169 41" stroke-width="5" opacity=".3"/>
      <path class="arc-a" style="--o:.85" d="M150 175 A52 52 0 0 1 106 175" stroke-width="7" opacity=".85"/>
      <path class="arc-b" style="--o:.55" d="M159 195 A74 74 0 0 1 97 195" stroke-width="6" opacity=".55"/>
      <path class="arc-c" style="--o:.3" d="M169 215 A96 96 0 0 1 87 215" stroke-width="5" opacity=".3"/>
    </g>
    <!-- claws -->
    <g fill="none" stroke="#F74C00" stroke-width="13" stroke-linecap="round">
      <path d="M9.5 -16.5 A19 19 0 1 1 -9.5 -16.5" transform="translate(62,128) rotate(90)"/>
      <path d="M9.5 -16.5 A19 19 0 1 1 -9.5 -16.5" transform="translate(194,128) rotate(-90)"/>
    </g>
    <!-- the neutron star -->
    <circle cx="128" cy="128" r="30" fill="url(#starGlow)"/>
    <circle cx="128" cy="128" r="12" fill="var(--star-core, #FFF7EE)"/>
  </symbol>
</svg>

![img](./assets/Crab_HubbleChandraSpitzer_1080.jpg)
The Spinning Pulsar of the Crab Nebula (*source: https://apod.nasa.gov/apod/ap220821.html*)
