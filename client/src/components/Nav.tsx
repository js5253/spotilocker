export function Nav() {
  return (
    <div class="navbar bg-base-100 shadow-sm">
      <div class="flex-1">
        <a href="/" class="btn btn-ghost text-xl text-green-400">
          spotilocker.
        </a>
      </div>
      <div class="flex-none">
        <ul class="menu menu-horizontal px-1">
          <li>
            <a>Get Started</a>
          </li>
          <li>
            <details>
              <summary>Parent</summary>
              <ul class="bg-base-100 rounded-t-none p-2">
                <li>
                  <a>Log In</a>
                </li>
                <li>
                  <a>Link 2</a>
                </li>
              </ul>
            </details>
          </li>
        </ul>
      </div>
    </div>
  );
}
