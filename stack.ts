import * as cdk from '@aws-cdk/core';
import * as lambda from '@aws-cdk/aws-lambda';
import * as s3 from '@aws-cdk/aws-s3';
import * as dynamo from '@aws-cdk/aws-dynamodb';
import * as path from 'path';
import * as sqs from '@aws-cdk/aws-sqs';
import { DynamoEventSource, SqsEventSource } from '@aws-cdk/aws-lambda-event-sources';

class MediaStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // DynamoDB
    const table = new dynamo.Table(this, `media-dynamo`, {
      tableName: `media-dynamo`,
      partitionKey: { name: 'pk', type: dynamo.AttributeType.STRING },
      sortKey: { name: 'sk', type: dynamo.AttributeType.STRING },
      stream: dynamo.StreamViewType.NEW_IMAGE,
    });

    // SQS
    const queue = new sqs.Queue(this, `media-sqs`, {
      queueName: `media-sqs`,
      visibilityTimeout: cdk.Duration.seconds(500),
    });

    // S3
    const bucket = new s3.Bucket(this, `media-s3-patrick`, {
      blockPublicAccess: s3.BlockPublicAccess.BLOCK_ALL,
      bucketName: `media-s3-patrick`,
    });

    // Lambdas
    const addImageDownloadToQueue = new lambda.Function(this, `AddImageDownloadToQueue`, {
      handler: `main`,
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/add_image_download_to_queue/bootstrap.zip`)),
      functionName: `AddImageDownloadToQueue`,
    });
    addImageDownloadToQueue.addEventSource(
      new DynamoEventSource(table, {
        startingPosition: lambda.StartingPosition.LATEST,
      })
    );

    // const downloadImageToS3 = new lambda.Function(this, `DownloadImageToS3`, {
    //   handler: `main`,
    //   runtime: lambda.Runtime.PROVIDED_AL2,
    //   code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/download_image_to_s3/bootstrap.zip`)),
    //   functionName: `DownloadImageToS3`,
    //   timeout: cdk.Duration.seconds(500),
    // });
    // downloadImageToS3.addEventSource(
    //   new SqsEventSource(queue, {
    //     batchSize: 1,
    //   })
    // );

    const writeWidescreenWallpapersToDynamo = new lambda.Function(this, `WriteWidescreenWallpapersToDynamo`, {
      handler: `main`,
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/widescreen_wallpapers/bootstrap.zip`)),
      functionName: `WriteWidescreenWallpapersToDynamo`,
    });

    // Permissions
    table.grantReadWriteData(writeWidescreenWallpapersToDynamo);
    queue.grantSendMessages(addImageDownloadToQueue);
    queue.grantConsumeMessages(addImageDownloadToQueue);
    // queue.grantConsumeMessages(downloadImageToS3);
    // bucket.grantWrite(downloadImageToS3);
  }
}

const app = new cdk.App();
new MediaStack(app, 'MediaStack', {});
