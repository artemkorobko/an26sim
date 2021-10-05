import React from 'react';
import './Application.css';
import { Grid } from 'semantic-ui-react';
import { Decoder } from './decoder/Decoder';
import { Encoder } from './encoder/Encoder';

class Application extends React.Component {
  render() {
    return (
      <Grid columns='equal' divided className="stretch-vertical">
        <Grid.Row className="top-spaced">
          <Grid.Column>
            <Decoder />
          </Grid.Column>
          <Grid.Column>
            <Encoder />
          </Grid.Column>
        </Grid.Row>
        123
      </Grid>
    );
  }
}

export default Application;
