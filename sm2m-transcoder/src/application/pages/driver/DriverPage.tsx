import React from "react";
import { Grid, Header, Label, Segment, SemanticCOLORS } from "semantic-ui-react";
import { BoardDetailedStatus, BoardStatus, DriverStatus } from "./Status";

import "./DriverPage.css";

interface DriverPageProps {
  driver?: DriverStatus,
  board?: BoardStatus,
}

export const DriverPage: React.FC<DriverPageProps> = (props: DriverPageProps) => {
  const renderDriverNotLoaded = () => {
    return <Label color="red">
      Driver is not loaded
    </Label>;
  }

  const renderError = (message: string) => {
    return <Label color="red">
      {message}
    </Label>;
  }

  const renderDriverStatus = (status?: DriverStatus) => {
    if (status) {
      if (status.error) {
        return renderError(status.error);
      } else {
        return <Label color="green">
          {status.version?.driver}
          <Label.Detail>
            {status.version?.libusb}
          </Label.Detail>
        </Label>;
      }
    } else {
      return renderDriverNotLoaded();
    }
  }

  const renderDecoderStatus = (status?: BoardDetailedStatus) => {
    if (status) {
      if (status.error) {
        return renderError(status.error);
      } else if (!status.connected) {
        return <Label color="red">
          Board not found
          <Label.Detail>
            retry in: {status.retryBarrierSec} sec
          </Label.Detail>
        </Label>;
      } else {
        return <>status</>;
      }
    } else {
      return renderDriverNotLoaded();
    }
  }

  const renderComponentBlock = (title: string, color: SemanticCOLORS, status: JSX.Element) => {
    return <Segment color={color}>
      <Grid>
        <Grid.Row>
          <Grid.Column width={10} verticalAlign="middle">
            <Header as="h4">
              {title}
            </Header>
          </Grid.Column>
          <Grid.Column width={6} textAlign="right">
            {status}
          </Grid.Column>
        </Grid.Row>
      </Grid>
    </Segment>;
  }

  return <>
    {renderComponentBlock("Driver", "grey", renderDriverStatus(props.driver))}
    {renderComponentBlock("Decoder", "blue", renderDecoderStatus(props.board?.decoder))}
    {renderComponentBlock("Encoder", "orange", renderDecoderStatus(props.board?.encoder))}
  </>;
}
