import React from "react";
import { Container } from "semantic-ui-react";

import "./TitleBar.css";

interface TitleBarProps {
  title: string;
}

export const TitleBar: React.FC<TitleBarProps> = (props: TitleBarProps) => {
  return (
    <Container textAlign="center" className="title-bar">
      {props.title}
    </Container>
  );
}
