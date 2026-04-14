import { UserProfile } from "../components/UserProfile";

export const ViewList = () => {
  const listTitle = null;
  const songs = [];
  return (
    <div class="p-4">
      <h2 class="text-4xl">{listTitle ? listTitle : "View List"}</h2>
      <UserProfile />
      {songs.length === 0 && <p>No Songs Found!</p>}
      {songs.map((song) => (
        <p>AAAAA</p>
      ))}
    </div>
  );
};
