import React, { useEffect, useState } from "react";
import { Container, Grid } from "semantic-ui-react";
import { HashRouter, Redirect, Route, Switch } from "react-router-dom";
import { invoke } from "@tauri-apps/api/tauri";
import { PageProps, PagesMenu } from "../components/pages-menu/PagesMenu";
import { DriverPage } from "./pages/driver/DriverPage";
import { DecoderPage } from "./pages/decoder/DecoderPage";
import { EncoderPage } from "./pages/encoder/EncoderPage";
import { StatusBar } from "../components/status-bar/StatusBar";
import { DriverStatus, BoardStatus, BoardDetailedStatus } from "./pages/driver/Status";
import { Version } from "../domain";

import "./Application.css";

const BOARD_DETECT_BARRIER_SEC = 10;

export const Application: React.FC = () => {
  const [driverStatus, setDriverStatus] = useState<DriverStatus>();
  const [boardStatus, setBoardStatus] = useState<BoardStatus>();
  let boardDetectTimer: number | null = null;

  useEffect(() => {
    invoke<Version>("driver_init")
      .then(version => {
        setDriverStatus({ version });
        setBoardStatus(buildDefaultBoardStatus());
        // retryDetectBoard();
      })
      .catch(error => setDriverStatus({ error }));

    return () => {
      stopDetectBoard();
    };
  }, [setDriverStatus, setBoardStatus]);

  const retryDetectBoard = () => {
    if (boardDetectTimer == null) {
      boardDetectTimer = window.setInterval(tryDetectBoards, 1000);
    }
  }

  const stopDetectBoard = () => {
    if (boardDetectTimer) {
      window.clearInterval(boardDetectTimer);
      boardDetectTimer = null;
    }
  }

  const tryDetectBoards = () => {
    if (boardStatus) {
      if (!boardStatus.encoder.connected) {
        tryDetectEncoder(boardStatus.encoder);
      }

      if (!boardStatus.decoder.connected) {
        tryDetectDecoder(boardStatus.decoder);
      }

      setBoardStatus(boardStatus);
      console.log("update:", boardStatus);
    }

    console.log("try");
  }

  const tryDetectEncoder = (status: BoardDetailedStatus) => {
    let retryBarrierSec = status.retryBarrierSec - 1;
    if (retryBarrierSec <= 0) {
      status.retryBarrierSec = BOARD_DETECT_BARRIER_SEC;
    } else {
      status.retryBarrierSec = retryBarrierSec;
    }
  }

  const tryDetectDecoder = (status: BoardDetailedStatus) => {
    let retryBarrierSec = status.retryBarrierSec - 1;
    if (retryBarrierSec <= 0) {
      retryBarrierSec = BOARD_DETECT_BARRIER_SEC;
    }
  }

  const buildDefaultBoardStatus = (): BoardStatus => {
    return {
      encoder: {
        connected: false,
        retryBarrierSec: BOARD_DETECT_BARRIER_SEC,
      },
      decoder: {
        connected: false,
        retryBarrierSec: BOARD_DETECT_BARRIER_SEC,
      }
    };
  }

  const buildPageProps = (): PageProps[] => [
    { name: "driver", enabled: true },
    { name: "decoder", enabled: boardStatus?.decoder?.error?.length === 0 || false },
    { name: "encoder", enabled: boardStatus?.encoder?.error?.length === 0 || false },
  ]

  return (
    <HashRouter>
      <Container>
        <Grid>
          <Grid.Row className="pages-menu-container">
            <Grid.Column textAlign="center">
              <PagesMenu pages={buildPageProps()} />
            </Grid.Column>
          </Grid.Row>
          <Grid.Row>
            <Grid.Column>
              <Switch>
                <Route exact path="/">
                  <Redirect to="/driver" />
                </Route>
                <Route exact path="/driver">
                  <DriverPage driver={driverStatus} board={boardStatus} />
                </Route>
                <Route exact path="/decoder">
                  <DecoderPage />
                </Route>
                <Route exact path="/encoder">
                  <EncoderPage />
                </Route>
              </Switch>
            </Grid.Column>
          </Grid.Row>
        </Grid>
      </Container>
      <StatusBar version={driverStatus?.version} />
    </HashRouter>
  );
}
