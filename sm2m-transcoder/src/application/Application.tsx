import React, { useState } from "react";
import { Container, Grid, Segment } from "semantic-ui-react";
import { HashRouter, Redirect, Route } from "react-router-dom";
import { PageProps, PagesMenu } from "../components/pages-menu/PagesMenu";
import { TitleBar } from "../components/title-bar/TitleBar";
import { StatusPage } from "./pages/status/StatusPage";
import { DecoderPage } from "./pages/decoder/DecoderPage";
import { EncoderPage } from "./pages/encoder/EncoderPage";

import "./Application.css";
import { StatusBar } from "../components/status-bar/StatusBar";

export const Application: React.FC = () => {
  const [hasDecoder, setHasDecoder] = useState<boolean>(true);
  const [hasEncoder, setHasEncoder] = useState<boolean>(true);

  const buildPageProps = (): PageProps[] => [
    { name: "status", disabled: false },
    { name: "decoder", disabled: !hasDecoder },
    { name: "encoder", disabled: !hasEncoder },
  ]

  return (
    <HashRouter>
      <Container>
        <Grid>
          <Grid.Row>
            <Grid.Column>
              <TitleBar title="SM2M Transcoder" />
            </Grid.Column>
          </Grid.Row>
          <Grid.Row>
            <Grid.Column textAlign="center">
              <PagesMenu pages={buildPageProps()} />
            </Grid.Column>
          </Grid.Row>
          <Grid.Row>
            <Grid.Column>
              <Route exact path="/">
                <Redirect to="/status" />
              </Route>
              <Route exact path="/status">
                <StatusPage />
              </Route>
              <Route exact path="/decoder">
                <DecoderPage />
              </Route>
              <Route exact path="/encoder">
                <EncoderPage />
              </Route>
            </Grid.Column>
          </Grid.Row>
        </Grid>
      </Container>
      <StatusBar />
    </HashRouter>
  );
}
