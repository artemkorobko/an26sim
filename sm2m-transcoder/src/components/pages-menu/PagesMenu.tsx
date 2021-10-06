import React, { useCallback, useState } from "react";
import { Link } from "react-router-dom";
import { Menu, MenuItemProps } from "semantic-ui-react";

import "./PagesMenu.css"

export interface PageProps {
  readonly name: string;
  readonly disabled: boolean;
}

interface PagesMenuProps {
  readonly pages: PageProps[];
}

export const PagesMenu: React.FC<PagesMenuProps> = (props: PagesMenuProps) => {
  const [activePage, setActivePage] = useState<string>("");

  const isActive = (name: string): boolean => activePage === name;
  const handleClick = useCallback((_, props: MenuItemProps) => setActivePage(props.name!), [setActivePage]);

  const renderMenuItems = (items: PageProps[]) => {
    return items.map(item =>
      <Menu.Item key={item.name} name={item.name} disabled={item.disabled} as={Link} to={item.name} active={isActive(item.name)} onClick={handleClick} />
    );
  }

  return (
    <Menu secondary compact>
      {renderMenuItems(props.pages)}
    </Menu>
  );
}
