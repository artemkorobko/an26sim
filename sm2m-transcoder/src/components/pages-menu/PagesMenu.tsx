import React, { useState } from "react";
import { NavLink } from "react-router-dom";
import { Menu } from "semantic-ui-react";

import "./PagesMenu.css"

export interface PageProps {
  readonly name: string;
  readonly enabled: boolean;
}

interface PagesMenuProps {
  readonly pages: PageProps[];
}

export const PagesMenu: React.FC<PagesMenuProps> = (props: PagesMenuProps) => {
  const renderMenuItems = (items: PageProps[]) => {
    return items.map(item =>
      <Menu.Item key={item.name} name={item.name} disabled={!item.enabled} as={NavLink} to={item.name} />
    );
  }

  return (
    <Menu secondary compact>
      {renderMenuItems(props.pages)}
    </Menu>
  );
}
