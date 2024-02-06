export function Header() {
  return (
    <header className="outline-dark flex w-full justify-between gap-4 p-4">
      <button className="outline-dark-hover w-full">Commit</button>
      <button className="outline-dark-hover w-full">Branches</button>
      <button className="outline-dark-hover w-full">History</button>
      <button className="outline-dark-hover w-full">Flow</button>
    </header>
  );
}
