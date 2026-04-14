export const HomeRoute = () => {
  return (
    <>
      <div class="hero bg-blue-400 min-h-[600px]">
        <div class="mx-auto text-center space-y-2">
          <h1 class="hero-content text-5xl font-bold">
            Save, publish, and transfer your Spotify liked.
          </h1>
          <p>
            An open-source tool to store, publish, and transfer playlists and
            songs.
          </p>
          <button class="btn">Sign In</button>
        </div>
      </div>
      <footer class="footer p-4 text-center w-full bg-gray-500">
        (this project is not affiliated with Spotify)
      </footer>
    </>
  );
};
